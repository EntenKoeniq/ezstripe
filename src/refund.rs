use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

/// Refunds object from 01/12/2023
/// 
/// [Refunds object](https://stripe.com/docs/api/refunds/object)
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  /// Unique identifier for the object.
  pub id: String,
  /// String representing the object’s type. Objects of the same type share the same value.
  pub object: String, // refund
  /// Amount, in cents.
  pub amount: u32,
  /// Balance transaction that describes the impact on your account balance.
  pub balance_transaction: Option<String>,
  /// ID of the charge that was refunded.
  pub charge: String,
  /// Time at which the object was created. Measured in seconds since the Unix epoch.
  pub created: i64,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
  pub metadata: HashMap<String, String>,
  /// ID of the PaymentIntent that was refunded.
  pub payment_intent: String,
  /// Reason for the refund, either user-provided (`duplicate`, `fraudulent`, or `requested_by_customer`) or generated by Stripe internally (`expired_uncaptured_charge`).
  pub reason: Option<String>,
  /// This is the transaction number that appears on email receipts sent for this refund.
  pub receipt_number: Option<String>,
  /// The transfer reversal that is associated with the refund.
  /// Only present if the charge came from another Stripe account.
  /// See the Connect documentation for details.
  pub source_transfer_reversal: Option<String>,
  /// Status of the refund.
  /// For credit card refunds, this can be `pending`, `succeeded`, or `failed`. For other types of refunds, it can be `pending`, `requires_action`, `succeeded`, `failed`, or `canceled`. Refer to our refunds documentation for more details.
  pub status: String,
  /// If the accompanying transfer was reversed, the transfer reversal object. Only applicable if the charge was created using the destination parameter.
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
          if r["object"] == "refund" {
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
          if r["object"] == "refund" {
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