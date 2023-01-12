/// Tip: Store this client in a lifetime variable
/// 
/// Use this Client to send requests to Stripe's API.
pub struct Client {
  /// The Stripe API uses API keys to authenticate requests.
  /// You can view and manage your API keys in the [Stripe Dashboard](https://stripe.com/login?redirect=/account/apikeys).
  pub secret_key: String
}

include!("split/implementations/client/balance.rs");

include!("split/implementations/client/mandate.rs");

include!("split/implementations/client/payment_intent.rs");

include!("split/implementations/client/payout.rs");

include!("split/implementations/client/refund.rs");

