use serde::Deserialize;

pub mod create;
pub mod capture;
pub mod retrieve;
pub mod cancel;

#[derive(Deserialize)]
/// Payment intent object from 01/08/2023
/// 
/// [Payment intent object](https://stripe.com/docs/api/payment_intents/create#payment_intent_object)
pub struct StripeResponse {
  pub id: String,
  pub object: String,
  pub amount: u32,
  pub amount_capturable: u32,
  //pub amount_details: ?,
  pub amount_received: u32,
  pub application: Option<String>,
  pub application_fee_amount: Option<u32>,
  //pub automatic_payment_methods: ?,
  pub canceled_at: Option<i64>,
  pub cancellation_reason: Option<String>,
  pub capture_method: String,
  pub client_secret: String,
  pub confirmation_method: String,
  pub created: i64,
  pub currency: String,
  pub customer: Option<String>,
  pub description: Option<String>,
  pub invoice: Option<String>,
  //pub last_payment_error: ?,
  pub latest_charge: Option<String>,
  pub livemode: bool,
  //pub metadata: ?,
  //pub next_action: ?,
  pub on_behalf_of: Option<String>,
  pub payment_method: Option<String>,
  //pub payment_method_options: ?,
  pub payment_method_types: Vec<String>,
  //pub processing: ?,
  pub receipt_email: Option<String>,
  pub review: Option<String>,
  //pub setup_future_usage: ?,
  //pub shipping: ?,
  pub statement_descriptor: Option<String>,
  pub statement_descriptor_suffix: Option<String>,
  pub status: String,
  //pub transfer_data: ?,
  pub transfer_group: Option<String>
}