use serde::{ Serialize, Deserialize };

include!("split/structs/balance_transaction/response.rs");

include!("split/structs/balance_transaction/response_list.rs");

#[derive(PartialEq)]
pub(crate) enum Types {
  RETRIEVE(String),
  LIST(String)
}

const BALANCE_TRANSACTION_URL: &str = "https://api.stripe.com/v1/balance_transactions";

impl Types {
  pub(crate) fn create_get_request(&self, client: &reqwest::Client, secret: &str)-> reqwest::RequestBuilder {
    let mut result = client
      .get(self._get_url())
      .basic_auth(secret, None::<&str>)
      .header("Content-Type", "application/x-www-form-urlencoded");
    
    if let Some(r) = self._get_body() {
      result = result.body(r);
    }

    result
  }

  fn _get_url(&self) -> String {
    match self {
      Self::RETRIEVE(id) => format!("{}/{}", BALANCE_TRANSACTION_URL, id),
      Self::LIST(_) => format!("{}", BALANCE_TRANSACTION_URL)
    }
  }

  fn _get_body(&self) -> Option<String> {
    let body = match self {
      Self::LIST(body) => body,
      _ => ""
    };

    if body.is_empty() {
      None
    } else {
      Some(body.to_string())
    }
  }
}

pub struct Info<'a> {
  pub(crate) r#type: Types,
  pub(crate) secret_key: String,
  pub(crate) reqwest_client: &'a reqwest::Client
}

impl Info<'_> {
  /// Sends a "GET" request to Stripe's API.
  pub async fn get(&self) -> Result<Response, (String, Option<crate::error::Info>)> {
    match self.r#type {
      Types::RETRIEVE(_) => (),
      Types::LIST(_) => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `get()`. Please use the `get_list()` function");
        }
        return Err(("This function is not compatible with the selected type".to_string(), None));
      }
    };
    
    crate::helper::make_reqwest::<Response>(self.r#type.create_get_request(self.reqwest_client, &self.secret_key)).await
  }

  /// Sends a "GET" request to Stripe's API.
  pub async fn get_list(&self) -> Result<ResponseList, (String, Option<crate::error::Info>)> {
    match self.r#type {
      Types::LIST(_) => (),
      Types::RETRIEVE(_) => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `get_list()`. Please use the `get()` function");
        }
        return Err(("This function is not compatible with the selected type".to_string(), None));
      }
    };
    
    crate::helper::make_reqwest::<ResponseList>(self.r#type.create_get_request(self.reqwest_client, &self.secret_key)).await
  }
}