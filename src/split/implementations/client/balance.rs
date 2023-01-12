impl Client {
  /// # Example
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///   let client = ezstripe::Client {
  ///     secret_key: "YOUR_SECRET_KEY".to_string()
  ///   };
  ///   
  ///   let stripe_response = client.retrieve_balance().send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn retrieve_balance(&self) -> crate::balance::Info {
    crate::balance::Info {
      secret_key: self.secret_key.clone()
    }
  }
}