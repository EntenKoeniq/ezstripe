impl Client {
  /// # Example
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///   let client = ezstripe::Client::new("SECRET_KEY");
  ///   
  ///   let stripe_response = client.retrieve_balance().get().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn retrieve_balance(&self) -> crate::balance::Info {
    crate::balance::Info {
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }
}