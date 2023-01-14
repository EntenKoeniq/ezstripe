use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

/// Funds that are available to be transferred or paid out, whether automatically by Stripe or explicitly via the [Transfers API](https://stripe.com/docs/api/balance#transfers) or [Payouts API](https://stripe.com/docs/api/balance#payouts).
/// The available balance for each currency and payment type can be found in the `source_types` property.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Available {
  /// Balance amount.
  pub amount: u32,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// Breakdown of balance by source types.
  pub source_types: HashMap<String, u32>
}

/// Funds that are not yet available in the balance, due to the 7-day rolling pay cycle.
/// The pending balance for each currency, and for each payment type, can be found in the `source_types` property.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pending {
  /// Balance amount.
  pub amount: u32,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// Breakdown of balance by source types.
  pub source_types: HashMap<String, u32>
}

/// Funds held due to negative balances on connected Custom accounts.
/// The connect reserve balance for each currency and payment type can be found in the `source_types` property.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConnectReserved {
  /// Balance amount.
  pub amount: u32,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// Breakdown of balance by source types.
  pub source_types: HashMap<String, u32>
}

/// Funds that can be paid out using Instant Payouts.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstantAvailable {
  /// Balance amount.
  pub amount: u32,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// Breakdown of balance by source types.
  pub source_types: HashMap<String, u32>
}

/// Funds that are available for use.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IssuingAvailable {
  /// Balance amount.
  pub amount: u32,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// Breakdown of balance by source types.
  pub source_types: HashMap<String, u32>
}

/// Funds that can be spent on your [Issued Cards](https://stripe.com/docs/api/balance/balance_object#issuing/cards).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Issuing {
  /// Funds that are available for use.
  pub available: Vec<IssuingAvailable>
}

/// Balance object from 01/12/2023
/// 
/// [Balance object](https://stripe.com/docs/api/balance/balance_object)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
  /// String representing the object’s type. Objects of the same type share the same value.
  pub object: String, // Value is "balance"
  /// Funds that are available to be transferred or paid out, whether automatically by Stripe or explicitly via the [Transfers API](https://stripe.com/docs/api/balance#transfers) or [Payouts API](https://stripe.com/docs/api/balance#payouts).
  /// The available balance for each currency and payment type can be found in the `source_types` property.
  pub available: Vec<Available>,
  /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
  pub livemode: bool,
  /// Funds that are not yet available in the balance, due to the 7-day rolling pay cycle.
  /// The pending balance for each currency, and for each payment type, can be found in the `source_types` property.
  pub pending: Vec<Pending>,
  /// Funds held due to negative balances on connected Custom accounts.
  /// The connect reserve balance for each currency and payment type can be found in the `source_types` property.
  pub connect_reserved: Option<Vec<ConnectReserved>>,
  /// Funds that can be paid out using Instant Payouts.
  pub instant_available: Option<Vec<InstantAvailable>>,
  /// Funds that can be spent on your [Issued Cards](https://stripe.com/docs/api/balance/balance_object#issuing/cards).
  pub issuing: Option<Issuing>
}

#[doc(hidden)]
pub struct Info {
  pub secret_key: String
}

impl Info {
  /// Sends a "GET" request to Stripe's API.
  pub async fn get(&self) -> Result<Response, (String, Option<crate::error::Info>)> {
    let crequest = reqwest::Client::new()
      .get("https://api.stripe.com/v1/balance")
      .basic_auth(&self.secret_key, None::<&str>)
      .header("Content-Type", "application/x-www-form-urlencoded");
    crate::helper::get_request::<Response>(crequest).await
  }
}
