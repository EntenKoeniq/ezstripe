use serde::{ Serialize, Deserialize };

/// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerAcceptanceOffline;

/// If this is a Mandate accepted online, this hash contains details about the online acceptance.
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerAcceptanceOnline {
  /// The IP address from which the Mandate was accepted by the customer.
  pub ip_address: String,
  ///The user agent of the browser from which the Mandate was accepted by the customer.
  pub user_agent: String
}

/// Details about the customer’s acceptance of the mandate.
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerAcceptance {
  /// The time at which the customer accepted the Mandate.
  pub accepted_at: i64,
  /// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
  pub offline: Option<CustomerAcceptanceOffline>,
  /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
  pub online: Option<CustomerAcceptanceOnline>,
  /// The type of customer acceptance information included with the Mandate. One of `online` or `offline`.
  pub r#type: String
}

/// If this is a `multi_use` mandate, this hash contains details about the mandate.
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiUse;

/// If this mandate is associated with a `acss_debit` payment method, this hash contains mandate information specific to the `acss_debit` payment method.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodDetailsAcssDebit {
  /// List of Stripe products where this mandate can be selected automatically.
  pub default_for: Vec<String>,
  /// Description of the interval.
  /// Only required if the ‘payment_schedule’ parameter is ‘interval’ or ‘combined’.
  pub interval_description: String,
  /// Payment schedule for the mandate.
  pub payment_schedule: String,
  /// Transaction type of the mandate.
  pub transaction_type: String
}

/// If this mandate is associated with a `au_becs_debit` payment method, this hash contains mandate information specific to the `au_becs_debit` payment method.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodDetailsAuBecsDebit {
  /// The URL of the mandate. This URL generally contains sensitive information about the customer and should be shared with them exclusively.
  pub url: String
}

/// If this mandate is associated with a `bacs_debit` payment method, this hash contains mandate information specific to the `bacs_debit` payment method.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodDetailsBacsDebit {
  /// The status of the mandate on the Bacs network. Can be one of `pending`, `revoked`, `refused`, or `accepted`.
  pub network_status: String,
  /// The unique reference identifying the mandate on the Bacs network.
  pub reference: String,
  /// The URL that will contain the mandate that the customer has signed.
  pub url: String
}

/// Details for off-session mandates.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodDetailsBlikOffSession {
  /// Amount of each recurring payment.
  pub amount: u32,
  /// Currency of each recurring payment.
  pub currency: String,
  /// Frequency interval of each recurring payment.
  pub interval: String,
  /// Frequency indicator of each recurring payment.
  pub interval_count: u16
}

/// If this mandate is associated with a `blik` payment method, this hash contains mandate information specific to the `blik` payment method.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodDetailsBlik {
  /// Date at which the mandate expires.
  pub expires_after: i64,
  /// Details for off-session mandates.
  pub off_session: PaymentMethodDetailsBlikOffSession,
  /// Type of the mandate.
  pub r#type: String,
}

/// If this mandate is associated with a `card` payment method, this hash contains mandate information specific to the `card` payment method.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodDetailsCard;

/// If this mandate is associated with a `link` payment method, this hash contains mandate information specific to the `link` payment method.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodDetailsLink;

/// If this mandate is associated with a `sepa_debit` payment method, this hash contains mandate information specific to the `sepa_debit` payment method.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodDetailsSepaDebit {
  /// The unique reference of the mandate.
  pub reference: String,
  /// The URL of the mandate. This URL generally contains sensitive information about the customer and should be shared with them exclusively.
  pub url: String
}

/// If this mandate is associated with a `us_bank_account` payment method, this hash contains mandate information specific to the `us_bank_account` payment method.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodDetailsUsBankAccount;

/// Additional mandate information specific to the payment method type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodDetails {
  /// If this mandate is associated with a `acss_debit` payment method, this hash contains mandate information specific to the `acss_debit` payment method.
  pub acss_debit: Option<PaymentMethodDetailsAcssDebit>,
  /// If this mandate is associated with a `au_becs_debit` payment method, this hash contains mandate information specific to the `au_becs_debit` payment method.
  pub au_becs_debit: Option<PaymentMethodDetailsAuBecsDebit>,
  /// If this mandate is associated with a `bacs_debit` payment method, this hash contains mandate information specific to the `bacs_debit` payment method.
  pub bacs_debit: Option<PaymentMethodDetailsBacsDebit>,
  /// If this mandate is associated with a `blik` payment method, this hash contains mandate information specific to the `blik` payment method.
  pub blik: Option<PaymentMethodDetailsBlik>,
  /// If this mandate is associated with a `card` payment method, this hash contains mandate information specific to the `card` payment method.
  pub card: Option<PaymentMethodDetailsCard>,
  /// If this mandate is associated with a `link` payment method, this hash contains mandate information specific to the `link` payment method.
  pub link: Option<PaymentMethodDetailsLink>,
  /// If this mandate is associated with a `sepa_debit` payment method, this hash contains mandate information specific to the `sepa_debit` payment method.
  pub sepa_debit: Option<PaymentMethodDetailsSepaDebit>,
  /// The type of the payment method associated with this mandate. An additional hash is included on `payment_method_details` with a name matching this value. It contains mandate information specific to the payment method.
  pub r#type: String,
  /// If this mandate is associated with a `us_bank_account` payment method, this hash contains mandate information specific to the `us_bank_account` payment method.
  pub us_bank_account: Option<PaymentMethodDetailsUsBankAccount>
}

/// If this is a `single_use` mandate, this hash contains details about the mandate.
#[derive(Serialize, Deserialize, Debug)]
pub struct SingleUse {
  /// On a single use mandate, the amount of the payment.
  pub amount: u32,
  /// On a single use mandate, the currency of the payment.
  pub currency: String
}

/// Mandate object from 01/12/2023
/// 
/// [Mandate object](https://stripe.com/docs/api/mandates/object)
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  /// Unique identifier for the object.
  pub id: String,
  /// String representing the object’s type. Objects of the same type share the same value.
  pub object: String, // mandate
  /// Details about the customer’s acceptance of the mandate.
  pub customer_acceptance: CustomerAcceptance,
  /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
  pub livemode: bool,
  /// If this is a `multi_use` mandate, this hash contains details about the mandate.
  pub multi_use: Option<MultiUse>,
  /// ID of the payment method associated with this mandate.
  pub payment_method: String,
  /// Additional mandate information specific to the payment method type.
  pub payment_method_details: PaymentMethodDetails,
  /// The status of the mandate, which indicates whether it can be used to initiate a payment.
  pub status: String,
  /// The type of the mandate.
  pub r#type: String,
  /// If this is a `single_use` mandate, this hash contains details about the mandate.
  pub single_use: Option<String> // Possible
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
pub struct Info {
  pub id: String,
  pub secret_key: String
}

impl Info {
  /// Send a `get` request to Stripe's API.
  pub async fn get(&self) -> Result<Vec<Response>, (String, Option<crate::error::Info>)> {
    let request = reqwest::Client::new()
      .get(format!("https://api.stripe.com/v1/mandates/{}", self.id))
      .basic_auth(&self.secret_key, None::<&str>)
      .header("Content-Type", "application/x-www-form-urlencoded")
      .send()
      .await;
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
          if r["object"] == "mandate" {
            if let Some(r2) = crate::helper::value_to_response::<Response>(r) {
              return Ok(vec![r2]);
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