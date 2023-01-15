/// Use `Client::new("SECRET_KEY")` to create a new `Client`.
///
/// Tip: Store this client in a lifetime variable to reuse it.
pub struct Client {
  /// The Stripe API uses API keys to authenticate requests.
  /// You can view and manage your API keys in the [Stripe Dashboard](https://stripe.com/login?redirect=/account/apikeys).
  pub secret_key: String,
  // The `reqwest::Client` used to make requests to Stripe's API.
  #[doc(hidden)]
  pub reqwest_client: reqwest::Client
}

#[cfg(feature = "balance")]
include!("split/implementations/client/balance.rs");

#[cfg(feature = "mandate")]
include!("split/implementations/client/mandate.rs");

#[cfg(feature = "payment_intent")]
include!("split/implementations/client/payment_intent.rs");

#[cfg(feature = "payout")]
include!("split/implementations/client/payout.rs");

#[cfg(feature = "refund")]
include!("split/implementations/client/refund.rs");

impl Client {
  /// Create a new `Client`.
  pub fn new(secret_key: &str) -> Self {
    Self {
      secret_key: secret_key.to_string(),
      reqwest_client: reqwest::Client::new()
    }
  }
}