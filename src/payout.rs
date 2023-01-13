use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

/// Payout object from 01/12/2023
/// 
/// [Payout object](https://stripe.com/docs/api/payouts/object)
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  /// Unique identifier for the object.
  pub id: String,
  /// String representing the object’s type.
  /// Objects of the same type share the same value.
  pub object: String, // payout
  /// Amount (in cents) to be transferred to your bank account or debit card.
  pub amount: u32,
  /// Date the payout is expected to arrive in the bank.
  /// This factors in delays like weekends or bank holidays.
  pub arrival_date: i64,
  /// Returns `true` if the payout was created by an [automated payout schedule](https://stripe.com/docs/payouts#payout-schedule), and `false` if it was [requested manually](https://stripe.com/docs/payouts#manual-payouts).
  pub automatic: bool,
  /// ID of the balance transaction that describes the impact of this payout on your account balance.
  pub balance_transaction: String,
  /// Time at which the object was created. Measured in seconds since the Unix epoch.
  pub created: i64,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// An arbitrary string attached to the object. Often useful for displaying to users.
  pub description: String,
  /// ID of the bank account or card the payout was sent to.
  pub destination: String,
  /// If the payout failed or was canceled, this will be the ID of the balance transaction that reversed the initial balance transaction, and puts the funds from the failed payout back in your balance.
  pub failure_balance_transaction: Option<String>,
  /// Error code explaining reason for payout failure if available. See [Types of payout failures](https://stripe.com/docs/api#payout_failures) for a list of failure codes.
  pub failure_code: Option<String>,
  /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
  pub livemode: bool,
  /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
  pub metadata: HashMap<String, String>,
  /// The method used to send this payout, which can be standard or instant. instant is only supported for payouts to debit cards. (See [Instant payouts for marketplaces](https://stripe.com/blog/instant-payouts-for-marketplaces) for more information.)
  pub method: String,
  /// If the payout reverses another, this is the ID of the original payout.
  pub original_payout: Option<String>,
  /// If the payout was reversed, this is the ID of the payout that reverses this payout.
  pub reversed_by: Option<String>,
  /// The source balance this payout came from. One of `card`, `fpx`, or `bank_account`.
  pub source_type: String,
  /// Extra information about a payout to be displayed on the user’s bank statement.
  pub statement_descriptor: Option<String>,
  /// Current status of the payout: `paid`, `pending`, `in_transit`, `canceled` or `failed`.
  /// A payout is `pending` until it is submitted to the bank, when it becomes `in_transit`.
  /// The status then changes to paid if the transaction goes through, or to `failed` or `canceled` (within 5 business days).
  /// Some failed payouts may initially show as `paid` but then change to `failed`.
  pub status: String,
  /// Can be `bank_account` or `card`.
  pub r#type: String
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
pub struct Info {
  pub r#type: Types,
  pub secret_key: String
}

impl Info {
  /// Send a `post` request to Stripe's API.
  pub async fn send(&self) -> Result<Response, (String, Option<crate::error::Info>)> {
    if log::log_enabled!(log::Level::Warn) {
      match self.r#type {
        Types::RETRIEVE(_) => log::warn!("Please use the `get()` function for `RETRIEVE`"),
        Types::LIST(_) => log::warn!("Please use the `get()` function for `LIST`"),
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
        if log::log_enabled!(log::Level::Error) {
          log::error!("Discovered errors! Send us this error so we can fix it (https://github.com/EntenKoeniq/ezstripe/issues)");
          log::error!("{}", e);
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

  /// Send a `get` request to Stripe's API.
  pub async fn get(&self) -> Result<Vec<Response>, (String, Option<crate::error::Info>)> {
    if log::log_enabled!(log::Level::Warn) {
      match self.r#type {
        Types::RETRIEVE(_) | Types::LIST(_) => (),
        _ => log::warn!("Please use the `send()` function for types other than `RETRIEVE` or `LIST`")
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
        if log::log_enabled!(log::Level::Error) {
          log::error!("Discovered errors! Send us this error so we can fix it (https://github.com/EntenKoeniq/ezstripe/issues)");
          log::error!("{}", e);
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
}
