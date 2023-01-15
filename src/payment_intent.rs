//! The response details are incomplete!
//! 
//! Help us to complete the response details on [Github](https://github.com/EntenKoeniq/ezstripe/pulls) <3

use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

include!("split/structs/payment_intent/response.rs");

/// Returns a list of all refunds you’ve previously created.
/// The refunds are returned in sorted order, with the most recent refunds appearing first.
/// For convenience, the 10 most recent refunds are always available by default on the charge object.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseList {
  /// String representing the object’s type.
  /// Objects of the same type share the same value.
  pub object: String,
  /// The end of the requested URL of the API.
  pub url: String,
  /// If more than the data received now exists `true` otherwise `false`.
  pub has_more: bool,
  /// All received data.
  pub data: Vec<Response>
}

#[doc(hidden)]
#[derive(PartialEq)]
pub enum Types {
  CREATE(String),
  RETRIEVE(String),
  CONFIRM(String, String),
  CANCEL(String, String),
  UPDATE(String, String),
  CAPTURE(String),
  LIST(String)
}

#[doc(hidden)]
const PAYMENT_INTENT_URL: &str = "https://api.stripe.com/v1/payment_intents";

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
      Self::CREATE(_) => format!("{}", PAYMENT_INTENT_URL),
      Self::RETRIEVE(id) => format!("{}/{}", PAYMENT_INTENT_URL, id),
      Self::CONFIRM(id, _) => format!("{}/{}/confirm", PAYMENT_INTENT_URL, id),
      Self::CANCEL(id, _) => format!("{}/{}/cancel", PAYMENT_INTENT_URL, id),
      Self::UPDATE(id, _) => format!("{}/{}", PAYMENT_INTENT_URL, id),
      Self::CAPTURE(id) => format!("{}/{}/capture", PAYMENT_INTENT_URL, id),
      Self::LIST(_) => format!("{}", PAYMENT_INTENT_URL)
    }
  }

  fn _get_body(&self) -> Option<String> {
    let body = match self {
      Self::CREATE(body) => body,
      Self::CONFIRM(_, body) => body,
      Self::CANCEL(_, body) => body,
      Self::UPDATE(_, body) => body,
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

/// This structure contains all the data for a request to Stripe's API.
pub struct Info<'a> {
  /// The type of request to Stripe.
  pub r#type: Types,
  /// Stripe's API secret key.
  pub secret_key: String,
  // A reference to the `reqwest::Client` reusable.
  #[doc(hidden)]
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