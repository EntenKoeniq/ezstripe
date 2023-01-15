/// Make a request with [reqwest](https://crates.io/crates/reqwest).
pub async fn make_reqwest<T>(request: reqwest::RequestBuilder) -> Result<T, (String, Option<crate::error::Info>)> where T : serde::de::DeserializeOwned {
  let response = match request.send().await {
      Ok(r) => r,
      Err(_) => return Err(("Request failed".to_string(), None))
    };
  
  let status = response.status();
  let body_response = match response.text().await {
    Ok(r) => r,
    Err(e) => {
      if log::log_enabled!(log::Level::Error) {
        log::error!("Discovered errors! Send us this error so we can fix it (https://github.com/EntenKoeniq/ezstripe/issues)");
        log::error!("{}", e);
      }
      return Err(("Body could not be unwrapped".to_string(), None));
    }
  };
  
  if status.is_success() {
    match serde_json::from_str::<T>(&body_response) {
      Ok(r) => return Ok(r),
      Err(e) => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("Discovered errors! Send us this error so we can fix it (https://github.com/EntenKoeniq/ezstripe/issues)");
          log::error!("{}", e);
        }
      }
    };
  } else {
    if let Some(r) = crate::error::Info::create(status.as_u16(), &body_response) {
      return Err(("Status is not success".to_string(), Some(r)));
    }
  }
  
  Err(("Something went wrong".to_string(), None))
}