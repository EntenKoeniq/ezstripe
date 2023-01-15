impl Client {
  /// # Arguments
  /// 
  /// * `id` - The unique ID of this mandate
  /// 
  /// # Example
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///   let client = ezstripe::Client::new("SECRET_KEY");
  ///   
  ///   let stripe_response = client.retrieve_mandate("ID_OF_MANDATE".to_string()).get().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn retrieve_mandate(&self, id: String) -> crate::mandate::Info {
    crate::mandate::Info {
      id,
      secret_key: self.secret_key.clone(),
      reqwest_client: &self.reqwest_client
    }
  }
}