impl Client {
  /// # Arguments
  /// 
  /// * `body` - The content that provides details for Stripe, e.g. B. Currency
  /// 
  /// # Example
  /// ```
  /// #[macro_use] extern crate ezstripe;
  /// 
  /// #[tokio::main]
  /// async fn main() {
  ///   let client = ezstripe::Client::new("SECRET_KEY");
  ///   
  ///   // Returns: String("amount=1500;currency=eur;payment_method_types[]=card;capture_method=manual;")
  ///   let stripe_body = ezbody!(
  ///       "amount" => 1500,
  ///       "currency" => "eur",
  ///       "payment_method_types[]" => "card",
  ///       "capture_method" => "manual"
  ///     );
  ///   
  ///   let stripe_response = client.create_payment_intent(stripe_body).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn create_payment_intent(&self, body: String) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::CREATE(body),
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }

  /// # Arguments
  /// 
  /// * `id` - The unique ID you received when you created it
  /// 
  /// # Example
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///   let client = ezstripe::Client::new("SECRET_KEY");
  ///   
  ///   let stripe_response = client.retrieve_payment_intent("PAYMENT_INTENT_ID".to_string()).get().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn retrieve_payment_intent(&self, id: String) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::RETRIEVE(id),
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }

  /// # Arguments
  /// 
  /// * `id` - The unique ID you received when you created it
  /// * `body` - The content that provides details for Stripe, e.g. B. Currency
  /// 
  /// # Example
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///   let client = ezstripe::Client::new("SECRET_KEY");
  ///   
  ///   let stripe_response = client.confirm_payment_intent("PAYMENT_INTENT_ID".to_string(), None).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn confirm_payment_intent(&self, id: String, body: Option<String>) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::CONFIRM(id, body.unwrap_or(String::default())),
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }

  /// # Arguments
  /// 
  /// * `id` - The unique ID you received when you created it
  /// * `body` - The content that provides details for Stripe, e.g. B. Currency
  /// 
  /// # Example
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///   let client = ezstripe::Client::new("SECRET_KEY");
  ///   
  ///   let stripe_response = client.cancel_payment_intent("PAYMENT_INTENT_ID".to_string(), None).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn cancel_payment_intent(&self, id: String, body: Option<String>) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::CANCEL(id, body.unwrap_or(String::default())),
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }

  /// # Arguments
  /// 
  /// * `id` - The unique ID you received when you created it
  /// * `body` - The content that provides details for Stripe, e.g. B. Currency
  /// 
  /// # Example
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///   let client = ezstripe::Client::new("SECRET_KEY");
  ///   
  ///   let stripe_response = client.update_payment_intent("PAYMENT_INTENT_ID".to_string(), "metadata[order_id]=2;".to_string()).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn update_payment_intent(&self, id: String, body: String) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::UPDATE(id, body),
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }

  /// # Arguments
  /// 
  /// * `id` - The unique ID you received when you created it
  /// 
  /// # Example
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///   let client = ezstripe::Client::new("SECRET_KEY");
  ///   
  ///   let stripe_response = client.capture_payment_intent("PAYMENT_INTENT_ID".to_string()).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn capture_payment_intent(&self, id: String) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::CAPTURE(id),
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }

  /// # Arguments
  /// 
  /// * `body` - The content that provides details for Stripe, e.g. B. Currency
  /// 
  /// # Example
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///   let client = ezstripe::Client::new("SECRET_KEY");
  ///   
  ///   let stripe_response = client.list_payment_intent("limit=3;".to_string()).get_list().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn list_payment_intent(&self, body: String) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::LIST(body),
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }
}