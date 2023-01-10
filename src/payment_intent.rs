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
  /// Unique identifier for the object.
  pub id: String,
  /// String representing the object’s type. Objects of the same type share the same value.
  pub object: String,
  /// Amount intended to be collected by this PaymentIntent.
  /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
  /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
  /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
  pub amount: u32,
  /// Amount that can be captured from this PaymentIntent.
  pub amount_capturable: u32,
  /// Details about items included in the amount
  pub amount_details: Option<AmountDetails>,
  /// Amount that was collected by this PaymentIntent.
  pub amount_received: u32,
  /// ID of the Connect application that created the PaymentIntent.
  pub application: Option<String>,
  /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner’s Stripe account.
  /// The amount of the application fee collected will be capped at the total payment amount.
  /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
  pub application_fee_amount: Option<u32>,
  /// Settings to configure compatible payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods)
  pub automatic_payment_methods: Option<AutomaticPaymentMethods>,
  /// Populated when `status` is `canceled`, this is the time at which the PaymentIntent was canceled.
  /// Measured in seconds since the Unix epoch.
  pub canceled_at: Option<i64>,
  /// Reason for cancellation of this PaymentIntent, either user-provided (`duplicate`, `fraudulent`, `requested_by_customer`, or `abandoned`) or generated by Stripe internally (`failed_invoice`, `void_invoice`, or `automatic`).
  pub cancellation_reason: Option<String>,
  /// Controls when the funds will be captured from the customer’s account.
  pub capture_method: String,
  /// The client secret of this PaymentIntent.
  /// Used for client-side retrieval using a publishable key.
  ///
  /// The client secret can be used to complete a payment from your frontend.
  /// It should not be stored, logged, or exposed to anyone other than the customer.
  /// Make sure that you have TLS enabled on any page that includes the client secret.
  /// 
  /// Refer to our docs to accept a payment and learn about how client_secret should be handled.
  pub client_secret: String,
  /// ?
  pub confirmation_method: String,
  /// Time at which the object was created. Measured in seconds since the Unix epoch.
  pub created: i64,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// ID of the Customer this PaymentIntent belongs to, if one exists.
  ///
  /// Payment methods attached to other Customers cannot be used with this PaymentIntent.
  ///
  /// If present in combination with [setup_future_usage](https://stripe.com/docs/api/errors#payment_intent_object-setup_future_usage), this PaymentIntent’s payment method will be attached to the Customer after the PaymentIntent has been confirmed and any required actions from the user are complete.
  pub customer: Option<String>,
  /// An arbitrary string attached to the object. Often useful for displaying to users.
  pub description: Option<String>,
  /// ID of the invoice that created this PaymentIntent, if it exists.
  pub invoice: Option<String>,
  /// The payment error encountered in the previous PaymentIntent confirmation. It will be cleared if the PaymentIntent is later updated for any reason.
  pub last_payment_error: Option<LastPaymentError>,
  /// The latest charge created by this payment intent.
  pub latest_charge: Option<String>,
  /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
  pub livemode: bool,
  //pub metadata: ?,
  //pub next_action: ?,
  /// The account (if any) for which the funds of the PaymentIntent are intended.
  /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
  pub on_behalf_of: Option<String>,
  /// ID of the payment method used in this PaymentIntent.
  pub payment_method: Option<String>,
  //pub payment_method_options: ?,
  /// The list of payment method types (e.g. card) that this PaymentIntent is allowed to use.
  pub payment_method_types: Vec<String>,
  //pub processing: ?,
  /// Email address that the receipt for the resulting payment will be sent to.
  /// If `receipt_email` is specified for a payment in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
  pub receipt_email: Option<String>,
  /// ID of the review associated with this PaymentIntent, if any.
  pub review: Option<String>,
  //pub setup_future_usage: ?,
  /// Shipping information for this PaymentIntent.
  pub shipping: Option<Shipping>,
  /// For non-card charges, you can use this value as the complete description that appears on your customers’ statements.
  /// Must contain at least one letter, maximum 22 characters.
  pub statement_descriptor: Option<String>,
  /// Provides information about a card payment that customers see on their statements.
  /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
  /// Maximum 22 characters for the concatenated descriptor.
  pub statement_descriptor_suffix: Option<String>,
  /// Status of this PaymentIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `requires_capture`, `canceled`, or `succeeded`.
  /// Read more about each PaymentIntent [status](https://stripe.com/docs/payments/intents#intent-statuses).
  pub status: String,
  /// The data with which to automatically create a Transfer when the payment is finalized.
  /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
  pub transfer_data: Option<TransferData>,
  /// A string that identifies the resulting payment as part of a group.
  /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
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

#[derive(PartialEq)]
pub enum Types {
  CREATE(String),
  RETRIEVE(String),
  CONFIRM(String, String),
  CANCEL(String, String),
  CAPTURE(String)
}

#[doc(hidden)]
const PAYMENT_INTENT_URL: &str = "https://api.stripe.com/v1/payment_intents";

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
      Self::CREATE(_) => format!("{}", PAYMENT_INTENT_URL),
      Self::RETRIEVE(id) => format!("{}/{}", PAYMENT_INTENT_URL, id),
      Self::CONFIRM(id, _) => format!("{}/{}/confirm", PAYMENT_INTENT_URL, id),
      Self::CANCEL(id, _) => format!("{}/{}/cancel", PAYMENT_INTENT_URL, id),
      Self::CAPTURE(id) => format!("{}/{}/capture", PAYMENT_INTENT_URL, id)
    }
  }

  fn _get_body(&self) -> Option<String> {
    let body = match self {
      Self::CREATE(body) => body,
      Self::CONFIRM(_, body) => body,
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
  /// Send a `post` request to Stripe's API.
  pub async fn send(&self) -> Result<crate::payment_intent::Response, Option<crate::error::Info>> {
    if matches!(self.r#type, Types::RETRIEVE(_)) {
      println!("[ezstripe]: {}Please use the `get()` function for `RETRIEVE`{}", "\x1b[0;31m", "\x1b[0m");
    }

    let request = self.r#type.create_send_request(&self.secret_key).send().await;
    if request.is_err() {
      return Err(None);
    }
  
    let response = request.unwrap();
    if response.status().is_success() {
      match response.json::<crate::payment_intent::Response>().await {
        Ok(r) => return Ok(r),
        Err(e) => {
          println!("[ezstripe]: {}Discovered errors! Send us this error so we can fix it (https://github.com/xEntenKoeniqx/ezstripe/issues){}", "\x1b[0;31m", "\x1b[0m");
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

  /// Send a `get` request to Stripe's API.
  pub async fn get(&self) -> Result<crate::payment_intent::Response, Option<crate::error::Info>> {
    if !matches!(self.r#type, Types::RETRIEVE(_)) {
      println!("[ezstripe]: {}Please use the `send()` function for types other than `RETRIEVE`{}", "\x1b[0;31m", "\x1b[0m");
    }

    let request = self.r#type.create_get_request(&self.secret_key).send().await;
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