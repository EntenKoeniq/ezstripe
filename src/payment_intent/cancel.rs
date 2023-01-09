pub struct Info {
  /// The ID of the PaymentIntent.
  pub id: String,
  /// [Look at all possible parameters of this request](https://stripe.com/docs/api/payment_intents/cancel#cancel_payment_intent-parameters).
  pub body: Option<String>
}

impl Info {
  /// Make a request to Stripe's API.
  pub async fn send(self) -> Result<super::StripeResponse, Option<crate::error::Info>> {
    let url = format!("https://api.stripe.com/v1/payment_intents/{}/cancel", self.id);
    let response = reqwest::Client::new()
      .post(url)
      .basic_auth(crate::get_secret(), None::<&str>)
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(self.body.unwrap_or(String::default()))
      .send()
      .await;
  
    if response.is_err() {
      return Err(None);
    }
  
    let res = response.unwrap();
    if res.status().is_success() {
      if let Ok(r) = res.json::<super::StripeResponse>().await {
        return Ok(r);
      }
    } else {
      let status = res.status().as_u16();
      let body_response = res.text().await;
      if body_response.is_ok() {
        if let Some(r) = crate::error::Info::create(status, &body_response.unwrap()) {
          return Err(Some(r));
        }
      }
    }
    
    Err(None)
  }
}