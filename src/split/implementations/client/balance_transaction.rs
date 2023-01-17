impl Client {
  /// # Example
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///   let client = ezstripe::Client::new("SECRET_KEY");
  ///   
  ///   let stripe_response = client.retrieve_balance_transaction("ID".to_string()).get().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn retrieve_balance_transaction(&self, id: String) -> crate::balance_transaction::Info {
    crate::balance_transaction::Info {
      r#type: crate::balance_transaction::Types::RETRIEVE(id),
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }

  /// # Example
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///   let client = ezstripe::Client::new("SECRET_KEY");
  ///   
  ///   let stripe_response = client.list_balance_transaction("BODY".to_string()).get_list().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn list_balance_transaction(&self, body: String) -> crate::balance_transaction::Info {
    crate::balance_transaction::Info {
      r#type: crate::balance_transaction::Types::LIST(body),
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }
}