/// Contains instructions for authenticating a payment by redirecting your customer to Alipay App or website.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionAlipayHandleRedirect {
  /// The native data to be used with Alipay SDK you must redirect your customer to in order to authenticate the payment in an Android App.
  pub native_data: String,
  /// The native URL you must redirect your customer to in order to authenticate the payment in an iOS App.
  pub native_url: String,
  /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
  pub return_url: String,
  /// The URL you must redirect your customer to in order to authenticate the payment.
  pub url: String
}

/// Contains Boleto details necessary for the customer to complete the payment.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionBoletoDisplayDetails {
  /// The timestamp after which the boleto expires.
  pub expires_at: i64,
  /// The URL to the hosted boleto voucher page, which allows customers to view the boleto voucher.
  pub hosted_voucher_url: String,
  /// The boleto number.
  pub number: String,
  /// The URL to the downloadable boleto voucher PDF.
  pub pdf: String
}

/// Contains instructions for processing off session recurring payments with Indian issued cards.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionCardAwaitNotification {
  /// The time that payment will be attempted.
  /// If customer approval is required, they need to provide approval before this time.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub charge_attempt_at: Option<i64>,
  /// For payments greater than INR 15000, the customer must provide explicit approval of the payment with their bank.
  /// For payments of lower amount, no customer action is required.
  pub customer_approval_required: bool
}

/// An IBAN-based FinancialAddress
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionDisplayBankTransferInstructionsFinancialAddressesIban {
  /// The name of the person or business that owns the bank account
  pub account_holder_name: String,
  /// The BIC/SWIFT code of the account.
  pub bic: String,
  /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
  pub country: String,
  /// The IBAN of the account.
  pub iban: String
}

/// An account number and sort code-based FinancialAddress
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionDisplayBankTransferInstructionsFinancialAddressesSortCode {
  /// The name of the person or business that owns the bank account
  pub account_holder_name: String,
  /// The account number
  pub account_number: String,
  /// The six-digit sort code
  pub sort_code: String
}

/// A SPEI-based FinancialAddress
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionDisplayBankTransferInstructionsFinancialAddressesSpei {
  /// The three-digit bank code
  pub bank_code: String,
  ///The short banking institution name
  pub bank_name: String,
  /// The CLABE number
  pub clabe: String
}

/// A Zengin-based FinancialAddress
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionDisplayBankTransferInstructionsFinancialAddressesZengin {
  /// The account holder name
  pub account_holder_name: String,
  /// The account number
  pub account_number: String,
  /// The bank account type. In Japan, this can only be `futsu` or `toza`.
  pub account_type: String,
  /// The bank code of the account
  pub bank_code: String,
  /// The bank name of the account
  pub bank_name: String,
  /// The branch code of the account
  pub branch_code: String,
  /// The branch name of the account
  pub branch_name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionDisplayBankTransferInstructionsFinancialAddresses {
  /// An IBAN-based FinancialAddress
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iban: Option<NextActionDisplayBankTransferInstructionsFinancialAddressesIban>,
  /// An account number and sort code-based FinancialAddress
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sort_code: Option<NextActionDisplayBankTransferInstructionsFinancialAddressesSortCode>,
  /// A SPEI-based FinancialAddress
  #[serde(skip_serializing_if = "Option::is_none")]
  pub spei: Option<NextActionDisplayBankTransferInstructionsFinancialAddressesSpei>,
  /// The payment networks supported by this FinancialAddress
  pub supported_networks: String,
  /// The type of financial address
  pub r#type: String,
  /// A Zengin-based FinancialAddress
  #[serde(skip_serializing_if = "Option::is_none")]
  pub zengin: Option<NextActionDisplayBankTransferInstructionsFinancialAddressesZengin>
}

/// Contains the bank transfer details necessary for the customer to complete the payment.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionDisplayBankTransferInstructions {
  /// The remaining amount that needs to be transferred to complete the payment.
  pub amount_remaining: u32,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// A list of financial addresses that can be used to fund the customer balance
  pub financial_addresses: Option<NextActionDisplayBankTransferInstructionsFinancialAddresses>,
  /// A link to a hosted page that guides your customer through completing the transfer.
  pub hosted_instructions_url: String,
  /// A string identifying this payment.
  /// Instruct your customer to include this code in the reference or memo field of their bank transfer.
  pub reference: String,
  /// Type of bank transfer
  pub r#type: String
}

/// Contains Konbini details necessary for the customer to complete the payment.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionKonbiniDisplayDetails;

/// Contains OXXO details necessary for the customer to complete the payment.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionOxxoDisplayDetails {
  /// The timestamp after which the OXXO voucher expires.
  pub expires_after: i64,
  /// The URL for the hosted OXXO voucher page, which allows customers to view and print an OXXO voucher.
  pub hosted_voucher_url: String,
  /// OXXO reference number.
  pub number: String
}

/// The field that contains PayNow QR code info
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionPaynowDisplayQrCode {
  /// The raw data string used to generate QR code, it should be used together with QR code library.
  pub data: String,
  /// The URL to the hosted PayNow instructions page, which allows customers to view the PayNow QR code.
  pub hosted_instructions_url: String,
  /// The image_url_png string used to render QR code
  pub image_url_png: String,
  /// The image_url_svg string used to render QR code
  pub image_url_svg: String
}

/// The field that contains Pix QR code info
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionPixDisplayQrCode {
  /// The raw data string used to generate QR code, it should be used together with QR code library.
  pub data: String,
  /// The date (unix timestamp) when the PIX expires.
  pub expires_at: i64,
  /// The URL to the hosted pix instructions page, which allows customers to view the pix QR code.
  pub hosted_instructions_url: String,
  /// The image_url_png string used to render png QR code
  pub image_url_png: String,
  /// The image_url_svg string used to render svg QR code
  pub image_url_svg: String
}

/// The field that contains PromptPay QR code info
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionPromptpayDisplayQrCode {
  /// The raw data string used to generate QR code, it should be used together with QR code library.
  pub data: String,
  /// The URL to the hosted PromptPay instructions page, which allows customers to view the PromptPay QR code.
  pub hosted_instructions_url: String,
  /// The PNG path used to render the QR code, can be used as the source in an HTML img tag
  pub image_url_png: String,
  /// The SVG path used to render the QR code, can be used as the source in an HTML img tag
  pub image_url_svg: String
}

/// Contains instructions for authenticating a payment by redirecting your customer to another page or application.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionRedirectToUrl {
  /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
  pub return_url: String,
  /// The URL you must redirect your customer to in order to authenticate the payment.
  pub url: String
}

/// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
/// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionUseStripeSdk;

/// Contains details describing microdeposits verification flow.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionVerifyWithMicrodeposits {
  /// The timestamp when the microdeposits are expected to land.
  pub arrival_date: i64,
  /// The URL for the hosted verification page, which allows customers to verify their bank account.
  pub hosted_verification_url: String,
  /// The type of the microdeposit sent to the customer.
  /// Used to distinguish between different verification methods.
  pub microdeposit_type: String
}

/// The field that contains WeChat Pay QR code info
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionWechatPayDisplayQrCode {
  /// The data being used to generate QR code
  pub data: String,
  /// The URL to the hosted WeChat Pay instructions page, which allows customers to view the WeChat Pay QR code.
  pub hosted_instructions_url: String,
  /// The base64 image data for a pre-generated QR code
  pub image_data_url: String,
  /// The image_url_png string used to render QR code
  pub image_url_png: String,
  /// The image_url_svg string used to render QR code
  pub image_url_svg: String
}

/// Info required for android app to app redirect
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionWechatPayRedirectToAndrodApp {
  /// app_id is the APP ID registered on WeChat open platform
  pub app_id: String,
  /// nonce_str is a random string
  pub monce_str: String,
  /// package is static value
  pub package: String,
  /// an unique merchant ID assigned by WeChat Pay
  pub partner_id: String,
  /// an unique trading ID assigned by WeChat Pay
  pub prepay_id: String,
  /// A signature
  pub sign: String,
  /// Specifies the current time in epoch format
  pub timestamp: String
}

/// Info required for iOS app to app redirect
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextActionWechatPayRedirectToIosApp {
  /// An universal link that redirect to WeChat Pay app
  pub native_url: String
}

/// If present, this property tells you what actions you need to take in order for your customer to fulfill a payment using the provided source.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NextAction {
  /// Contains instructions for authenticating a payment by redirecting your customer to Alipay App or website.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub alipay_handle_redirect: Option<NextActionAlipayHandleRedirect>,
  /// Contains Boleto details necessary for the customer to complete the payment.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub boleto_display_details: Option<NextActionBoletoDisplayDetails>,
  /// Contains instructions for processing off session recurring payments with Indian issued cards.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub card_await_notification: Option<NextActionCardAwaitNotification>,
  /// Contains the bank transfer details necessary for the customer to complete the payment.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub display_bank_transfer_instructions: Option<NextActionDisplayBankTransferInstructions>,
  /// Contains Konbini details necessary for the customer to complete the payment.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub konbini_display_details: Option<NextActionKonbiniDisplayDetails>,
  /// Contains OXXO details necessary for the customer to complete the payment.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub oxxo_display_details: Option<NextActionOxxoDisplayDetails>,
  /// The field that contains PayNow QR code info
  #[serde(skip_serializing_if = "Option::is_none")]
  pub paynow_display_qr_code: Option<NextActionPaynowDisplayQrCode>,
  /// The field that contains Pix QR code info
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pix_display_qr_code: Option<NextActionPixDisplayQrCode>,
  /// The field that contains PromptPay QR code info
  #[serde(skip_serializing_if = "Option::is_none")]
  pub promptpay_display_qr_code: Option<NextActionPromptpayDisplayQrCode>,
  /// Contains instructions for authenticating a payment by redirecting your customer to another page or application.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub redirect_to_url: Option<NextActionRedirectToUrl>,
  /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
  pub r#type: String,
  /// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
  /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub use_stripe_sdk: Option<NextActionUseStripeSdk>,
  /// Contains details describing microdeposits verification flow.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub verify_with_microdeposits: Option<NextActionVerifyWithMicrodeposits>,
  /// The field that contains WeChat Pay QR code info
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wechat_pay_display_qr_code: Option<NextActionWechatPayDisplayQrCode>,
  /// Info required for android app to app redirect
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wechat_pay_redirect_to_android_app: Option<NextActionWechatPayRedirectToAndrodApp>,
  /// Info required for iOS app to app redirect
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wechat_pay_redirect_to_ios_app: Option<NextActionWechatPayRedirectToIosApp>
}