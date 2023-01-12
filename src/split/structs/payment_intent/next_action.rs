/// Contains instructions for processing off session recurring payments with Indian issued cards.
#[derive(Serialize, Deserialize, Debug)]
pub struct NextActionCardAwaitNotification {
  /// The time that payment will be attempted. If customer approval is required, they need to provide approval before this time.
  pub charge_attempt_at: Option<i64>,
  /// For payments greater than INR 15000, the customer must provide explicit approval of the payment with their bank.
  /// For payments of lower amount, no customer action is required.
  pub customer_approval_required: Option<bool>
}

/// Contains instructions for authenticating a payment by redirecting your customer to another page or application.
#[derive(Serialize, Deserialize, Debug)]
pub struct NextActionRedirectToUrl {
  /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
  pub return_url: Option<String>,
  /// The URL you must redirect your customer to in order to authenticate the payment.
  pub url: Option<String>
}

/// If present, this property tells you what actions you need to take in order for your customer to fulfill a payment using the provided source.
/// 
/// MISSING DETAILS: `use_stripe_sdk`, `alipay_handle_redirect`, `boleto_display_details`, `display_bank_transfer_instructions`, `konbini_display_details`, `oxxo_display_details`, `paynow_display_qr_code`, `pix_display_qr_code`, `promptpay_display_qr_code`, `verify_with_microdeposits`, `wechat_pay_display_qr_code`, `wechat_pay_redirect_to_android_app`, `wechat_pay_redirect_to_ios_app`
#[derive(Serialize, Deserialize, Debug)]
pub struct NextAction {
  /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
  pub r#type: Option<String>,
  /// Contains instructions for processing off session recurring payments with Indian issued cards.
  pub card_await_notification: Option<NextActionCardAwaitNotification>,
  /// Contains instructions for authenticating a payment by redirecting your customer to another page or application.
  pub redirect_to_url: Option<NextActionRedirectToUrl>
}