use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct AmountDetailsTip {
  pub amount: Option<u32>
}

#[derive(Serialize, Deserialize)]
pub struct AmountDetails {
  pub tip: AmountDetailsTip
}

#[derive(Serialize, Deserialize)]
pub struct AutomaticPaymentMethods {
  pub enabled: bool
}

/// Has to be tested! May crash as some values ​​may be null
#[derive(Serialize, Deserialize)]
pub struct LastPaymentError {
  pub charge: String,
  pub code: String,
  pub decline_code: String,
  pub doc_url: String,
  pub message: String,
  pub param: String,
  //pub payment_method: ?,
  pub payment_method_type: String,
  pub r#type: String
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
  //pub shipping: ?,
  pub statement_descriptor: Option<String>,
  pub statement_descriptor_suffix: Option<String>,
  pub status: String,
  //pub transfer_data: ?,
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