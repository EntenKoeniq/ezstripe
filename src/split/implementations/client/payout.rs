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
  ///       "currency" => "eur"
  ///     );
  ///   
  ///   let stripe_response = client.create_payout(stripe_body).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn create_payout(&self, body: String) -> crate::payout::Info {
    crate::payout::Info {
      r#type: crate::payout::Types::CREATE(body),
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
  ///   let stripe_response = client.retrieve_payout("PAYOUT_ID".to_string()).get().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn retrieve_payout(&self, id: String) -> crate::payout::Info {
    crate::payout::Info {
      r#type: crate::payout::Types::RETRIEVE(id),
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
  ///   let stripe_response = client.update_payout("PAYOUT_ID".to_string(), "metadata[order_id]=2;".to_string()).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn update_payout(&self, id: String, body: String) -> crate::payout::Info {
    crate::payout::Info {
      r#type: crate::payout::Types::UPDATE(id, body),
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
  ///   let stripe_response = client.list_payout("limit=3;".to_string()).get_list().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn list_payout(&self, body: String) -> crate::payout::Info {
    crate::payout::Info {
      r#type: crate::payout::Types::LIST(body),
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
  ///   let stripe_response = client.cancel_payout("PAYOUT_ID".to_string()).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn cancel_payout(&self, id: String) -> crate::payout::Info {
    crate::payout::Info {
      r#type: crate::payout::Types::CANCEL(id),
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
  ///   let stripe_response = client.reverse_payout("PAYOUT_ID".to_string(), None).get().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn reverse_payout(&self, id: String, body: Option<String>) -> crate::payout::Info {
    crate::payout::Info {
      r#type: crate::payout::Types::REVERSE(id, body.unwrap_or(String::default())),
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }
}