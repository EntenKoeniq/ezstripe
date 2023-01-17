impl Client {
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
  ///   let stripe_response = client.retrieve_dispute("DISPUTE_ID".to_string()).get().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn retrieve_dispute(&self, id: String) -> crate::dispute::Info {
    crate::dispute::Info {
      r#type: crate::dispute::Types::RETRIEVE(id),
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
  ///   let stripe_response = client.update_dispute("DISPUTE_ID".to_string(), "metadata[order_id]=2;".to_string()).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn update_dispute(&self, id: String, body: String) -> crate::dispute::Info {
    crate::dispute::Info {
      r#type: crate::dispute::Types::UPDATE(id, body),
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
  ///   let stripe_response = client.close_dispute("DISPUTE_ID".to_string()).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn close_dispute(&self, id: String) -> crate::dispute::Info {
    crate::dispute::Info {
      r#type: crate::dispute::Types::CLOSE(id),
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
  ///   let stripe_response = client.list_dispute("limit=3;".to_string()).get_list().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn list_dispute(&self, body: String) -> crate::dispute::Info {
    crate::dispute::Info {
      r#type: crate::dispute::Types::LIST(body),
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }
}