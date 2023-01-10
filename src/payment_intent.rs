use serde::{ Serialize, Deserialize };

/// Portion of the amount that corresponds to a tip.
#[derive(Serialize, Deserialize)]
pub struct AmountDetailsTip {
  /// Portion of the amount that corresponds to a tip.
  pub amount: Option<u32>
}

/// Details about items included in the amount
#[derive(Serialize, Deserialize)]
pub struct AmountDetails {
  /// Portion of the amount that corresponds to a tip.
  pub tip: AmountDetailsTip
}

/// Settings to configure compatible payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods)
#[derive(Serialize, Deserialize)]
pub struct AutomaticPaymentMethods {
  /// Automatically calculates compatible payment methods
  pub enabled: bool
}

/// **Has to be tested! May crash as some values ​​may be null**
/// 
/// The payment error encountered in the previous PaymentIntent confirmation.
/// It will be cleared if the PaymentIntent is later updated for any reason.
#[derive(Serialize, Deserialize)]
pub struct LastPaymentError {
  /// For card errors, the ID of the failed charge.
  pub charge: String,
  /// For some errors that could be handled programmatically, a short string indicating the [error code](https://stripe.com/docs/error-codes) reported.
  pub code: String,
  /// For card errors resulting from a card issuer decline, a short string indicating the [card issuer’s reason for the decline](https://stripe.com/docs/declines#issuer-declines) if they provide one.
  pub decline_code: String,
  /// A URL to more information about the [error code](https://stripe.com/docs/error-codes) reported.
  pub doc_url: String,
  /// A human-readable message providing more details about the error.
  /// For card errors, these messages can be shown to your users.
  pub message: String,
  /// If the error is parameter-specific, the parameter related to the error.
  /// For example, you can use this to display a message near the correct form field.
  pub param: String,
  //pub payment_method: ?,
  /// If the error is specific to the type of payment method, the payment method type that had a problem.
  /// This field is only populated for invoice-related errors.
  pub payment_method_type: String,
  /// The type of error returned.
  /// One of `api_error`, `card_error`, `idempotency_error`, or `invalid_request_error`
  pub r#type: String
}

/// Shipping address.
#[derive(Serialize, Deserialize)]
pub struct ShippingAddress {
  /// City, district, suburb, town, or village.
  pub city: String,
  /// Two-letter country code [(ISO 3166-1 alpha-2)](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
  pub country: String,
  /// Address line 1 (e.g., street, PO Box, or company name).
  pub line1: String,
  /// Address line 2 (e.g., apartment, suite, unit, or building).
  pub line2: Option<String>, // can be null?
  /// ZIP or postal code.
  pub postal_code: String,
  /// State, county, province, or region.
  pub state: String
}

/// Shipping information for this PaymentIntent.
#[derive(Serialize, Deserialize)]
pub struct Shipping {
  /// Shipping address.
  pub address: ShippingAddress,
  /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
  pub carrier: String,
  /// Recipient name.
  pub name: String,
  /// Recipient phone (including extension).
  pub phone: Option<String>, // can be null?
  /// The tracking number for a physical product, obtained from the delivery service.
  /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
  pub tracking_number: Option<String> // can be null?
}

/// The data with which to automatically create a Transfer when the payment is finalized. See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
#[derive(Serialize, Deserialize)]
pub struct TransferData {
  ///Amount intended to be collected by this PaymentIntent.
  /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
  /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
  /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
  pub amount: u32,
  /// The account (if any) the payment will be attributed to for tax reporting, and where funds from the payment will be transferred to upon payment success.
  pub destination: Option<String>
}

/// Payment intent object from 01/08/2023
/// 
/// [Payment intent object](https://stripe.com/docs/api/payment_intents/create#payment_intent_object)
#[derive(Serialize, Deserialize)]
pub struct Response {
  pub id: String,
  pub object: String,
  pub amount: u32,
  pub amount_capturable: u32,
  pub amount_details: Option<AmountDetails>,
  pub amount_received: u32,
  pub application: Option<String>,
  pub application_fee_amount: Option<u32>,
  pub automatic_payment_methods: Option<AutomaticPaymentMethods>,
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
  pub last_payment_error: Option<LastPaymentError>,
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
  pub shipping: Option<Shipping>,
  pub statement_descriptor: Option<String>,
  pub statement_descriptor_suffix: Option<String>,
  pub status: String,
  pub transfer_data: Option<TransferData>,
  pub transfer_group: Option<String>
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

pub enum Types {
  CREATE(String),
  RETRIEVE(String),
  CANCEL(String, String),
  CAPTURE(String)
}

#[doc(hidden)]
impl Types {
  pub fn create_request(&self, secret: &str)-> reqwest::RequestBuilder {
    let mut result = reqwest::Client::new()
      .post(self._get_url())
      .basic_auth(secret, None::<&str>)
      .header("Content-Type", "application/x-www-form-urlencoded");
    
    if let Some(r) = self._get_body() {
      result = result.body(r);
    }

    result
  }

  fn _get_url(&self) -> String {
    match self {
      Self::CREATE(_) => format!("https://api.stripe.com/v1/payment_intents"),
      Self::RETRIEVE(id) => format!("https://api.stripe.com/v1/payment_intents/{}", id),
      Self::CANCEL(id, _) => format!("https://api.stripe.com/v1/payment_intents/{}/cancel", id),
      Self::CAPTURE(id) => format!("https://api.stripe.com/v1/payment_intents/{}/capture", id)
    }
  }

  fn _get_body(&self) -> Option<String> {
    let body = match self {
      Self::CREATE(body) => body,
      Self::CANCEL(_, body) => body,
      _ => ""
    };

    if body.is_empty() {
      None
    } else {
      Some(body.to_string())
    }
  }
}

pub struct Info {
  pub r#type: Types,
  pub secret_key: String
}

impl Info {
  /// Send a request to Stripe's API.
  pub async fn send(&self) -> Result<crate::payment_intent::Response, Option<crate::error::Info>> {
    let request = self.r#type.create_request(&self.secret_key).send().await;
    if request.is_err() {
      return Err(None);
    }
  
    let response = request.unwrap();
    if response.status().is_success() {
      match response.json::<crate::payment_intent::Response>().await {
        Ok(r) => return Ok(r),
        Err(e) => {
          println!("{}Discovered errors! Send us this error so we can fix it (https://github.com/xEntenKoeniqx/ezstripe/issues){}", "\x1b[0;31m", "\x1b[0m");
          println!("{}", e);
        }
      }
    } else {
      let status = response.status().as_u16();
      let body_response = response.text().await;
      if body_response.is_ok() {
        if let Some(r) = crate::error::Info::create(status, &body_response.unwrap()) {
          return Err(Some(r));
        }
      }
    }
    
    Err(None)
  }
}