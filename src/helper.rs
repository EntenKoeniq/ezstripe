pub fn value_to_response<T>(val: serde_json::Value) -> Option<T> where T : serde::de::DeserializeOwned {
  match serde_json::from_value::<T>(val) {
    Ok(r) => return Some(r),
    Err(e) => {
      if log::log_enabled!(log::Level::Error) {
        log::error!("Discovered errors! Send us this error so we can fix it (https://github.com/xEntenKoeniqx/ezstripe/issues)");
        log::error!("{}", e);
      }
    }
  };

  None
}

pub fn value_to_response_list<T>(val: serde_json::Value) -> Option<Vec<T>>  where T : serde::de::DeserializeOwned {
  match serde_json::from_value::<Vec<T>>(val) {
    Ok(r) => return Some(r),
    Err(e) => {
      if log::log_enabled!(log::Level::Error) {
        log::error!("Discovered errors! Send us this error so we can fix it (https://github.com/xEntenKoeniqx/ezstripe/issues)");
        log::error!("{}", e);
      }
    }
  };

  None
}