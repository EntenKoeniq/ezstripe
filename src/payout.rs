use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

/// Payout object from 01/12/2023
/// 
/// [Payout object](https://stripe.com/docs/api/payouts/object)
#[derive(Serialize, Deserialize, Clone, Debug)]
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

/// This structure contains all the data for a request to Stripe's API.
pub struct Info {
  /// The type of request to Stripe.
  pub r#type: Types,
  /// Stripe's API secret key.
  pub secret_key: String
}

impl Info {
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

    crate::helper::post_request::<Response>(self.r#type.create_send_request(&self.secret_key)).await
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
    
    crate::helper::get_request::<Response>(self.r#type.create_get_request(&self.secret_key)).await
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
    
    crate::helper::get_request::<ResponseList>(self.r#type.create_get_request(&self.secret_key)).await
  }
}
