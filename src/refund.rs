use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  pub id: String,
  pub object: String, // refund
  pub amount: u32,
  pub balance_transaction: Option<String>,
  pub charge: String,
  pub created: i64,
  pub currency: String,
  pub metadata: HashMap<String, String>,
  pub payment_intent: String,
  pub reason: Option<String>,
  pub receipt_number: Option<String>,
  pub source_transfer_reversal: Option<String>,
  pub status: String,
  pub transfer_reversal: Option<String>
}

impl Response {
  /// Get the complete Response as String.
  pub fn to_string(&self) -> Result<String, ()> {
    match serde_json::to_string(self) {
      Ok(r) => Ok(r),
      Err(_) => Err(())
    }
  }
}

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
const REFUND_URL: &str = "https://api.stripe.com/v1/refunds";

#[doc(hidden)]
impl Types {
  pub fn create_send_request(&self, secret: &str)-> reqwest::RequestBuilder {
    let mut result = reqwest::Client::new()
      .post(self._get_url())
      .basic_auth(secret, None::<&str>)
      .header("Content-Type", "application/x-www-form-urlencoded");
    
    if let Some(r) = self._get_body() {
      result = result.body(r);
    }

    result
  }

  pub fn create_get_request(&self, secret: &str)-> reqwest::RequestBuilder {
    let mut result = reqwest::Client::new()
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
      Self::CREATE(_) => format!("{}", REFUND_URL),
      Self::RETRIEVE(id) => format!("{}/{}", REFUND_URL, id),
      Self::UPDATE(id, _) => format!("{}/{}", REFUND_URL, id),
      Self::LIST(_) => format!("{}", REFUND_URL),
      Self::CANCEL(id) => format!("{}/{}/cancel", REFUND_URL, id),
      Self::REVERSE(id, _) => format!("{}/{}/reverse", REFUND_URL, id)
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

/// MISSING DOCUMENTATION
pub struct Info {
  /// MISSING DOCUMENTATION
  pub r#type: Types,
  /// MISSING DOCUMENTATION
  pub secret_key: String
}

impl Info {
  /// Send a `post` request to Stripe's API.
  pub async fn send(&self) -> Result<Response, (String, Option<crate::error::Info>)> {
    if crate::get_debug() {
      match self.r#type {
        Types::RETRIEVE(_) => println!("[ezstripe]: {}Please use the `get()` function for `RETRIEVE`{}", "\x1b[0;31m", "\x1b[0m"),
        Types::LIST(_) => println!("[ezstripe]: {}Please use the `get()` function for `LIST`{}", "\x1b[0;31m", "\x1b[0m"),
        _ => ()
      };
    }

    let request = self.r#type.create_send_request(&self.secret_key).send().await;
    if request.is_err() {
      return Err(("Request failed".to_string(), None));
    }
  
    let response = request.unwrap();
    let status = response.status();
    let body_response = match response.text().await {
      Ok(r) => r,
      Err(e) => {
        if crate::get_debug() {
          println!("[ezstripe]: {}Discovered errors! Send us this error so we can fix it (https://github.com/xEntenKoeniqx/ezstripe/issues){}", "\x1b[0;31m", "\x1b[0m");
          println!("{}", e);
        }
        return Err(("Body could not be unwrapped".to_string(), None));
      }
    };

    if status.is_success() {
      match serde_json::from_str::<serde_json::Value>(&body_response) {
        Ok(r) => {
          if r["object"] == "payout" {
            if let Some(r2) = crate::helper::value_to_response::<Response>(r) {
              return Ok(r2);
            }
          }
        },
        Err(e) => {
          if crate::get_debug() {
            println!("[ezstripe]: {}Discovered errors! Send us this error so we can fix it (https://github.com/xEntenKoeniqx/ezstripe/issues){}", "\x1b[0;31m", "\x1b[0m");
            println!("{}", e);
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

  /// Send a `get` request to Stripe's API.
  pub async fn get(&self) -> Result<Vec<Response>, (String, Option<crate::error::Info>)> {
    if crate::get_debug() {
      match self.r#type {
        Types::RETRIEVE(_) | Types::LIST(_) => (),
        _ => println!("[ezstripe]: {}Please use the `send()` function for types other than `RETRIEVE` or `LIST`{}", "\x1b[0;31m", "\x1b[0m")
      };
    }
    
    let request = self.r#type.create_get_request(&self.secret_key).send().await;
    if request.is_err() {
      return Err(("Request failed".to_string(), None));
    }
  
    let response = request.unwrap();
    let status = response.status();
    let body_response = match response.text().await {
      Ok(r) => r,
      Err(e) => {
        if crate::get_debug() {
          println!("[ezstripe]: {}Discovered errors! Send us this error so we can fix it (https://github.com/xEntenKoeniqx/ezstripe/issues){}", "\x1b[0;31m", "\x1b[0m");
          println!("{}", e);
        }
        return Err(("Body could not be unwrapped".to_string(), None));
      }
    };

    if status.is_success() {
      match serde_json::from_str::<serde_json::Value>(&body_response) {
        Ok(r) => {
          if r["object"] == "payout" {
            if let Some(r2) = crate::helper::value_to_response::<Response>(r) {
              return Ok(vec![r2]);
            }
          } else if r["object"] == "list" {
            if let Some(r2) = crate::helper::value_to_response_list::<Response>(r) {
              return Ok(r2);
            }
          }
        },
        Err(e) => {
          if crate::get_debug() {
            println!("[ezstripe]: {}Discovered errors! Send us this error so we can fix it (https://github.com/xEntenKoeniqx/ezstripe/issues){}", "\x1b[0;31m", "\x1b[0m");
            println!("{}", e);
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
}