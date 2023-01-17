use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

include!("split/structs/payout/response.rs");

include!("split/structs/payout/response_list.rs");

#[doc(hidden)]
#[derive(PartialEq)]
pub enum Types {
  CREATE(String),
  RETRIEVE(String),
  UPDATE(String, String),
  LIST(String),
  CANCEL(String),
  REVERSE(String, String)
}

#[doc(hidden)]
const PAYOUT_URL: &str = "https://api.stripe.com/v1/payouts";

#[doc(hidden)]
impl Types {
  pub fn create_send_request(&self, client: &reqwest::Client, secret: &str)-> reqwest::RequestBuilder {
    let mut result = client
      .post(self._get_url())
      .basic_auth(secret, None::<&str>)
      .header("Content-Type", "application/x-www-form-urlencoded");
    
    if let Some(r) = self._get_body() {
      result = result.body(r);
    }

    result
  }

  pub fn create_get_request(&self, client: &reqwest::Client, secret: &str)-> reqwest::RequestBuilder {
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
      Self::CREATE(_) => format!("{}", PAYOUT_URL),
      Self::RETRIEVE(id) => format!("{}/{}", PAYOUT_URL, id),
      Self::UPDATE(id, _) => format!("{}/{}", PAYOUT_URL, id),
      Self::LIST(_) => format!("{}", PAYOUT_URL),
      Self::CANCEL(id) => format!("{}/{}/cancel", PAYOUT_URL, id),
      Self::REVERSE(id, _) => format!("{}/{}/reverse", PAYOUT_URL, id)
    }
  }

  fn _get_body(&self) -> Option<String> {
    let body = match self {
      Self::CREATE(body) => body,
      Self::UPDATE(_, body) => body,
      Self::LIST(body) => body,
      Self::REVERSE(_, body) => body,
      _ => ""
    };

    if body.is_empty() {
      None
    } else {
      Some(body.to_string())
    }
  }
}

#[doc(hidden)]
pub struct Info<'a> {
  pub r#type: Types,
  pub secret_key: String,
  pub reqwest_client: &'a reqwest::Client
}

impl Info<'_> {
  /// Sends a "POST" request to Stripe's API.
  pub async fn send(&self) -> Result<Response, (String, Option<crate::error::Info>)> {
    match self.r#type {
      Types::RETRIEVE(_) => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `send()`. Please use the `get()` function");
        }
        return Err(("This function is not compatible with the selected type".to_string(), None));
      },
      Types::LIST(_) => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `send()`. Please use the `get_list()` function");
        }
        return Err(("This function is not compatible with the selected type".to_string(), None));
      },
      _ => ()
    };

    crate::helper::make_reqwest::<Response>(self.r#type.create_send_request(self.reqwest_client, &self.secret_key)).await
  }
  
  /// Sends a "GET" request to Stripe's API.
  pub async fn get(&self) -> Result<Response, (String, Option<crate::error::Info>)> {
    match self.r#type {
      Types::RETRIEVE(_) => (),
      Types::LIST(_) => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `get()`. Please use the `get_list()` function");
        }
        return Err(("This function is not compatible with the selected type".to_string(), None));
      },
      _ => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `get()`. Please use the `send()` function");
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
      },
      _ => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `get_list()`. Please use the `send()` function");
        }
        return Err(("This function is not compatible with the selected type".to_string(), None));
      }
    };
    
    crate::helper::make_reqwest::<ResponseList>(self.r#type.create_get_request(self.reqwest_client, &self.secret_key)).await
  }
}