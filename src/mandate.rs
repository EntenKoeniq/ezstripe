use serde::{ Serialize, Deserialize };

include!("split/structs/mandate/response.rs");

/// This structure contains all the data for a request to Stripe's API.
pub struct Info<'a> {
  /// Unique identifier of the object.
  pub id: String,
  /// Stripe's API secret key.
  pub secret_key: String,
  // A reference to the `reqwest::Client` reusable.
  #[doc(hidden)]
  pub reqwest_client: &'a reqwest::Client
}

impl Info<'_> {
  /// Sends a "GET" request to Stripe's API.
  pub async fn get(&self) -> Result<Response, (String, Option<crate::error::Info>)> {
    let crequest = self.reqwest_client
      .get(format!("https://api.stripe.com/v1/mandates/{}", self.id))
      .basic_auth(&self.secret_key, None::<&str>)
      .header("Content-Type", "application/x-www-form-urlencoded");
    crate::helper::make_reqwest::<Response>(crequest).await
  }
}