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
  ///   let client = ezstripe::Client {
  ///     secret_key: "YOUR_SECRET_KEY".to_string()
  ///   };
  ///   
  ///   // Returns: String("amount=1500;currency=eur;payment_method_types[]=card;capture_method=manual;")
  ///   let stripe_body = ezbody!(
  ///       "amount" => 1500,
  ///       "currency" => "eur"
  ///     );
  ///   
  ///   let stripe_response = client.create_refund(stripe_body).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn create_refund(&self, body: String) -> crate::refund::Info {
    crate::refund::Info {
      r#type: crate::refund::Types::CREATE(body),
      secret_key: self.secret_key.clone()
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
  ///   let client = ezstripe::Client {
  ///     secret_key: "YOUR_SECRET_KEY".to_string()
  ///   };
  ///   
  ///   let stripe_response = client.retrieve_refund("REFUND_ID".to_string()).get().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn retrieve_refund(&self, id: String) -> crate::refund::Info {
    crate::refund::Info {
      r#type: crate::refund::Types::RETRIEVE(id),
      secret_key: self.secret_key.clone()
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
  ///   let client = ezstripe::Client {
  ///     secret_key: "YOUR_SECRET_KEY".to_string()
  ///   };
  ///   
  ///   let stripe_response = client.update_refund("REFUND_ID".to_string(), "metadata[order_id]=2;".to_string()).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn update_refund(&self, id: String, body: String) -> crate::refund::Info {
    crate::refund::Info {
      r#type: crate::refund::Types::UPDATE(id, body),
      secret_key: self.secret_key.clone()
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
  ///   let client = ezstripe::Client {
  ///     secret_key: "YOUR_SECRET_KEY".to_string()
  ///   };
  ///   
  ///   let stripe_response = client.list_refund("limit=3;".to_string()).get().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn list_refund(&self, body: String) -> crate::refund::Info {
    crate::refund::Info {
      r#type: crate::refund::Types::LIST(body),
      secret_key: self.secret_key.clone()
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
  ///   let client = ezstripe::Client {
  ///     secret_key: "YOUR_SECRET_KEY".to_string()
  ///   };
  ///   
  ///   let stripe_response = client.cancel_refund("REFUND_ID".to_string()).send().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn cancel_refund(&self, id: String) -> crate::refund::Info {
    crate::refund::Info {
      r#type: crate::refund::Types::CANCEL(id),
      secret_key: self.secret_key.clone()
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
  ///   let client = ezstripe::Client {
  ///     secret_key: "YOUR_SECRET_KEY".to_string()
  ///   };
  ///   
  ///   let stripe_response = client.reverse_refund("REFUND_ID".to_string(), None).get().await;
  ///   
  ///   // ...
  /// }
  /// ```
  pub fn reverse_refund(&self, id: String, body: Option<String>) -> crate::refund::Info {
    crate::refund::Info {
      r#type: crate::refund::Types::REVERSE(id, body.unwrap_or(String::default())),
      secret_key: self.secret_key.clone()
    }
  }
}