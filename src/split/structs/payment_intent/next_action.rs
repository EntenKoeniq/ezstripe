/// Contains instructions for authenticating a payment by redirecting your customer to Alipay App or website.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionAlipayHandleRedirect;

/// Contains Boleto details necessary for the customer to complete the payment.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionBoletoDisplayDetails;

/// Contains instructions for processing off session recurring payments with Indian issued cards.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionCardAwaitNotification {
  /// The time that payment will be attempted.
  /// If customer approval is required, they need to provide approval before this time.
  pub charge_attempt_at: Option<i64>,
  /// For payments greater than INR 15000, the customer must provide explicit approval of the payment with their bank.
  /// For payments of lower amount, no customer action is required.
  pub customer_approval_required: Option<bool>
}

/// Contains the bank transfer details necessary for the customer to complete the payment.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionDisplayBankTransferInstructions;

/// Contains Konbini details necessary for the customer to complete the payment.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionKonbiniDisplayDetails;

/// Contains OXXO details necessary for the customer to complete the payment.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionOxxoDisplayDetails;

/// The field that contains PayNow QR code info
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionPaynowDisplayQrCode;

/// The field that contains Pix QR code info
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionPixDisplayQrCode;

/// The field that contains PromptPay QR code info
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionPromptpayDisplayQrCode;

/// Contains instructions for authenticating a payment by redirecting your customer to another page or application.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionRedirectToUrl {
  /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
  pub return_url: Option<String>,
  /// The URL you must redirect your customer to in order to authenticate the payment.
  pub url: Option<String>
}

/// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
/// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionUseStripeSdk;

/// Contains details describing microdeposits verification flow.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionVerifyWithMicrodeposits;

/// The field that contains WeChat Pay QR code info
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionWechatPayDisplayQrCode;

/// Info required for android app to app redirect
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionWechatPayRedirectToAndrodApp;

/// Info required for iOS app to app redirect
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionWechatPayRedirectToIosApp;

/// If present, this property tells you what actions you need to take in order for your customer to fulfill a payment using the provided source.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextAction {
  /// Contains instructions for authenticating a payment by redirecting your customer to Alipay App or website.
  pub alipay_handle_redirect: Option<NextActionAlipayHandleRedirect>,
  /// Contains Boleto details necessary for the customer to complete the payment.
  pub boleto_display_details: Option<NextActionBoletoDisplayDetails>,
  /// Contains instructions for processing off session recurring payments with Indian issued cards.
  pub card_await_notification: Option<NextActionCardAwaitNotification>,
  /// Contains the bank transfer details necessary for the customer to complete the payment.
  pub display_bank_transfer_instructions: Option<NextActionDisplayBankTransferInstructions>,
  /// Contains Konbini details necessary for the customer to complete the payment.
  pub konbini_display_details: Option<NextActionKonbiniDisplayDetails>,
  /// Contains OXXO details necessary for the customer to complete the payment.
  pub oxxo_display_details: Option<NextActionOxxoDisplayDetails>,
  /// The field that contains PayNow QR code info
  pub paynow_display_qr_code: Option<NextActionPaynowDisplayQrCode>,
  /// The field that contains Pix QR code info
  pub pix_display_qr_code: Option<NextActionPixDisplayQrCode>,
  /// The field that contains PromptPay QR code info
  pub promptpay_display_qr_code: Option<NextActionPromptpayDisplayQrCode>,
  /// Contains instructions for authenticating a payment by redirecting your customer to another page or application.
  pub redirect_to_url: Option<NextActionRedirectToUrl>,
  /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
  pub r#type: String,
  /// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
  /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
  pub use_stripe_sdk: Option<NextActionUseStripeSdk>,
  /// Contains details describing microdeposits verification flow.
  pub verify_with_microdeposits: Option<NextActionVerifyWithMicrodeposits>,
  /// The field that contains WeChat Pay QR code info
  pub wechat_pay_display_qr_code: Option<NextActionWechatPayDisplayQrCode>,
  /// Info required for android app to app redirect
  pub wechat_pay_redirect_to_android_app: Option<NextActionWechatPayRedirectToAndrodApp>,
  /// Info required for iOS app to app redirect
  pub wechat_pay_redirect_to_ios_app: Option<NextActionWechatPayRedirectToIosApp>
}