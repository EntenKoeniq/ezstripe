#[macro_use]
extern crate serde_json;

pub mod payment_intent;
pub mod error;

#[doc(hidden)]
pub static mut DEBUG: bool = false;
#[doc(hidden)]
pub fn get_debug() -> bool {
  unsafe {
    DEBUG
  }
}

/// **(STRONGLY RECOMMENDED IN DEVELOPMENT)**
/// 
/// Writes errors (if any) to your console.
/// 
/// # Arguments
/// 
/// * `val` - "true" if you want to enable it, "false" otherwise. (default: false)
/// 
/// # Example
/// ```
/// fn main() {
///   unsafe {
///     ezstripe::set_debug(true);
///   };
/// 
///   // ...
/// }
/// ```
pub unsafe fn set_debug(val: bool) {
  DEBUG = val;
}

/// Tip: Store this client in a lifetime variable
/// 
/// Use this Client to send requests to Stripe's API.
pub struct Client {
  /// The Stripe API uses API keys to authenticate requests.
  /// You can view and manage your API keys in the [Stripe Dashboard](https://stripe.com/login?redirect=/account/apikeys).
  pub secret_key: String
}

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
  ///     secret_key: "KEY".to_string()
  ///   };
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
  ///     secret_key: "KEY".to_string()
  ///   };
  /// 
  ///   let stripe_response = client.retrieve_payment_intent("PAYMENT_INTENT_ID".to_string()).get().await;
  /// 
  ///   // ...
  /// }
  /// ```
  pub fn retrieve_payment_intent(&self, id: String) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::RETRIEVE(id),
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
  ///     secret_key: "KEY".to_string()
  ///   };
  /// 
  ///   let stripe_response = client.confirm_payment_intent("PAYMENT_INTENT_ID".to_string(), None).send().await;
  /// 
  ///   // ...
  /// }
  /// ```
  pub fn confirm_payment_intent(&self, id: String, body: Option<String>) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::CONFIRM(id, body.unwrap_or(String::default())),
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
  ///     secret_key: "KEY".to_string()
  ///   };
  /// 
  ///   let stripe_response = client.cancel_payment_intent("PAYMENT_INTENT_ID".to_string(), None).send().await;
  /// 
  ///   // ...
  /// }
  /// ```
  pub fn cancel_payment_intent(&self, id: String, body: Option<String>) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::CANCEL(id, body.unwrap_or(String::default())),
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
  ///     secret_key: "KEY".to_string()
  ///   };
  /// 
  ///   let stripe_response = client.update_payment_intent("PAYMENT_INTENT_ID".to_string(), "metadata[order_id]=2;").send().await;
  /// 
  ///   // ...
  /// }
  /// ```
  pub fn update_payment_intent(&self, id: String, body: String) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::UPDATE(id, body),
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
  ///     secret_key: "KEY".to_string()
  ///   };
  /// 
  ///   let stripe_response = client.capture_payment_intent("PAYMENT_INTENT_ID".to_string()).send().await;
  /// 
  ///   // ...
  /// }
  /// ```
  pub fn capture_payment_intent(&self, id: String) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::CAPTURE(id),
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
  ///     secret_key: "KEY".to_string()
  ///   };
  /// 
  ///   let stripe_response = client.list_payment_intent("limit=3;".to_string()).get().await;
  /// 
  ///   // ...
  /// }
  /// ```
  pub fn list_payment_intent(&self, body: String) -> crate::payment_intent::Info {
    crate::payment_intent::Info {
      r#type: crate::payment_intent::Types::LIST(body),
      secret_key: self.secret_key.clone()
    }
  }
}

/// Create an easy body format for API requests.
/// 
/// # Example
/// ```
/// #[macro_use] extern crate ezstripe;
/// 
/// fn main() {
///   let body = ezbody!(
///       "amount" => 2000,
///       "currency" => "eur"
///     );
/// 
///   println!("{}", body); // amount=2000;currency=eur;
/// }
/// ```
#[macro_export]
macro_rules! ezbody {
  {$($k: expr => $v: expr),* $(,)?} => {
    {
      let mut result = String::new();

      $(
        result += format!("{}={};", $k, $v).as_str();
      )*

      result
    }
  };
}