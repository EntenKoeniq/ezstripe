use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

include!("split/structs/balance/response.rs");

pub struct Info<'a> {
  pub(crate) secret_key: String,
  pub(crate) reqwest_client: &'a reqwest::Client
}

impl Info<'_> {
  /// Sends a "GET" request to Stripe's API.
  pub async fn get(&self) -> Result<Response, (String, Option<crate::error::Info>)> {
    let crequest = self.reqwest_client
      .get("https://api.stripe.com/v1/balance")
      .basic_auth(&self.secret_key, None::<&str>)
      .header("Content-Type", "application/x-www-form-urlencoded");
    
    crate::helper::make_reqwest::<Response>(crequest).await
  }
}