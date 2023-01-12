use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Pending {
  pub amount: u32,
  pub currency: String,
  pub source_types: HashMap<String, u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Available {
  pub amount: u32,
  pub currency: String,
  pub source_types: HashMap<String, u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  pub object: String,
  pub available: Vec<Available>,
  pub livemode: bool,
  pub pending: Vec<Pending>
}

#[derive(PartialEq)]
pub enum Types {
  RETRIEVE
}

#[doc(hidden)]
const BALANCE_URL: &str = "https://api.stripe.com/v1/balance";

#[doc(hidden)]
impl Types {
  pub fn create_get_request(&self, secret: &str)-> reqwest::RequestBuilder {
    let mut result = reqwest::Client::new()
      .get(self._get_url())
      .basic_auth(secret, None::<&str>)
      .header("Content-Type", "application/x-www-form-urlencoded");
    
    //if let Some(r) = self._get_body() {
    //  result = result.body(r);
    //}

    result
  }

  fn _get_url(&self) -> String {
    match self {
      Self::RETRIEVE => format!("{}", BALANCE_URL)
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
  /// Send a `get` request to Stripe's API.
  pub async fn get(&self) -> Result<Vec<Response>, (String, Option<crate::error::Info>)> {
    if crate::get_debug() {
      match self.r#type {
        Types::RETRIEVE => (),
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
          if r["object"] == "balance" {
            if let Some(r2) = _value_to_response(r) {
              return Ok(vec![r2]);
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
fn _value_to_response(val: serde_json::Value) -> Option<Response> {
  match serde_json::from_value::<Response>(val) {
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
fn _value_to_response_list(val: serde_json::Value) -> Option<Vec<Response>> {
  match serde_json::from_value::<Vec<Response>>(val) {
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