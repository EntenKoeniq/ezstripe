//! The response details are incomplete!
//! 
//! Help us to complete the response details on [Github](https://github.com/xEntenKoeniqx/ezstripe/pulls) <3

use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

/// Portion of the amount that corresponds to a tip.
#[derive(Serialize, Deserialize, Debug)]
pub struct AmountDetailsTip {
  /// Portion of the amount that corresponds to a tip.
  pub amount: Option<u32>
}

/// Details about items included in the amount
#[derive(Serialize, Deserialize, Debug)]
pub struct AmountDetails {
  /// Portion of the amount that corresponds to a tip.
  pub tip: AmountDetailsTip
}

/// Settings to configure compatible payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods)
#[derive(Serialize, Deserialize, Debug)]
pub struct AutomaticPaymentMethods {
  /// Automatically calculates compatible payment methods
  pub enabled: bool
}

/// **Has to be tested! May crash as some values ​​may be null**
/// 
/// The payment error encountered in the previous PaymentIntent confirmation.
/// It will be cleared if the PaymentIntent is later updated for any reason.
/// 
/// MISSING DETAILS: `payment_method`
#[derive(Serialize, Deserialize, Debug)]
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

/// If the SetupIntent’s payment_method_types includes `card`, this hash contains the configurations that will be applied to each setup attempt of that type.
/// 
/// MISSING DETAILS: `mandate_options`
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsCard {
  //pub mandate_options: ?,
  /// Selected network to process this SetupIntent on. Depends on the available networks of the card attached to the setup intent.
  /// Can be only set confirm-time.
  pub network: Option<String>,
  /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
  /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
  /// Permitted values include: `automatic` or `any`.
  /// If not provided, defaults to `automatic`.
  /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
  pub request_three_d_secure: String
}

/// If the SetupIntent’s payment_method_types includes `link`, this hash contains the configurations that will be applied to each setup attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsLink {
  /// Token used for persistent Link logins.
  pub persistent_token: Option<String>
}

/// Payment-method-specific configuration for this SetupIntent.
/// 
/// MISSING DETAILS: `acss_debit`, `blik`, `sepa_debit` and `us_bank_account`
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptions {
  //pub acss_debit: ?,
  //pub blik: ?,
  /// If the SetupIntent’s payment_method_types includes `card`, this hash contains the configurations that will be applied to each setup attempt of that type.
  pub card: Option<PaymentMethodOptionsCard>,
  /// If the SetupIntent’s payment_method_types includes `link`, this hash contains the configurations that will be applied to each setup attempt of that type.
  pub link: Option<PaymentMethodOptionsLink>,
  //pub sepa_debit: ?,
  //pub us_bank_account: ?
}

/// Shipping address.
#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingAddress {
  /// City, district, suburb, town, or village.
  pub city: Option<String>,
  /// Two-letter country code [(ISO 3166-1 alpha-2)](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
  pub country: Option<String>,
  /// Address line 1 (e.g., street, PO Box, or company name).
  pub line1: Option<String>,
  /// Address line 2 (e.g., apartment, suite, unit, or building).
  pub line2: Option<String>, // can be null?
  /// ZIP or postal code.
  pub postal_code: Option<String>,
  /// State, county, province, or region.
  pub state: Option<String>
}

/// Shipping information for this PaymentIntent.
#[derive(Serialize, Deserialize, Debug)]
pub struct Shipping {
  /// Shipping address.
  pub address: ShippingAddress,
  /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
  pub carrier: Option<String>,
  /// Recipient name.
  pub name: Option<String>,
  /// Recipient phone (including extension).
  pub phone: Option<String>, // can be null?
  /// The tracking number for a physical product, obtained from the delivery service.
  /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
  pub tracking_number: Option<String> // can be null?
}

/// The data with which to automatically create a Transfer when the payment is finalized. See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
#[derive(Serialize, Deserialize, Debug)]
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
/// 
/// MISSING DETAILS: `metadata`, `next_action`, `processing` and `setup_future_usage`
#[derive(Serialize, Deserialize, Debug)]
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
  /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
  /// This can be useful for storing additional information about the object in a structured format.
  /// For more information, see the [documentation](https://stripe.com/docs/payments/payment-intents/creating-payment-intents#storing-information-in-metadata).
  pub metadata: HashMap<String, String>,
  //pub next_action: ?,
  /// The account (if any) for which the funds of the PaymentIntent are intended.
  /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
  pub on_behalf_of: Option<String>,
  /// ID of the payment method used in this PaymentIntent.
  pub payment_method: Option<String>,
  /// Payment-method-specific configuration for this SetupIntent.
  /// 
  /// ONLY `card` AND `link` ARE CURRENTLY SUPPORTED!
  pub payment_method_options: Option<PaymentMethodOptions>,
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
  UPDATE(String, String),
  CAPTURE(String),
  LIST(String)
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


/// MISSING DOCUMENTATION
pub struct Info {
  /// MISSING DOCUMENTATION
  pub r#type: Types,
  /// MISSING DOCUMENTATION
  pub secret_key: String
}

impl Info {
  /// Send a `post` request to Stripe's API.
  pub async fn send(&self) -> Result<crate::payment_intent::Response, (String, Option<crate::error::Info>)> {
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
          if r["object"] == "payment_intent" {
            if let Some(r2) = _value_to_response(r) {
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
  pub async fn get(&self) -> Result<Vec<crate::payment_intent::Response>, (String, Option<crate::error::Info>)> {
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
          if r["object"] == "payment_intent" {
            if let Some(r2) = _value_to_response(r) {
              return Ok(vec![r2]);
            }
          } else if r["object"] == "list" {
            if let Some(r2) = _value_to_response_list(r["data"].clone()) {
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

#[doc(hidden)]
fn _value_to_response(val: serde_json::Value) -> Option<crate::payment_intent::Response> {
  match serde_json::from_value::<crate::payment_intent::Response>(val) {
    Ok(r) => return Some(r),
    Err(e) => {
      if crate::get_debug() {
        println!("[ezstripe]: {}Discovered errors! Send us this error so we can fix it (https://github.com/xEntenKoeniqx/ezstripe/issues){}", "\x1b[0;31m", "\x1b[0m");
        println!("{}", e);
      }
    }
  };

  None
}

#[doc(hidden)]
fn _value_to_response_list(val: serde_json::Value) -> Option<Vec<crate::payment_intent::Response>> {
  match serde_json::from_value::<Vec<crate::payment_intent::Response>>(val) {
    Ok(r) => return Some(r),
    Err(e) => {
      if crate::get_debug() {
        println!("[ezstripe]: {}Discovered errors! Send us this error so we can fix it (https://github.com/xEntenKoeniqx/ezstripe/issues){}", "\x1b[0;31m", "\x1b[0m");
        println!("{}", e);
      }
    }
  };

  None
}