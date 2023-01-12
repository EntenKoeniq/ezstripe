/// Settings to configure compatible payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods)
#[derive(Serialize, Deserialize, Debug)]
pub struct AutomaticPaymentMethods {
  /// Automatically calculates compatible payment methods
  pub enabled: bool
}