/// Additional fields for Mandate creation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsAcssDebitMandateOptions {
  /// A URL for custom mandate text
  #[serde(skip_serializing_if = "Option::is_none")]
  pub custom_mandate_url: Option<String>,
  /// Description of the interval.
  /// Only required if the ‘payment_schedule’ parameter is ‘interval’ or ‘combined’.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub interval_description: Option<String>,
  /// Payment schedule for the mandate.
  pub payment_schedule: String,
  /// Transaction type of the mandate.
  pub transaction_type: String
}

/// If the PaymentIntent’s payment_method_types includes `acss_debit`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsAcssDebit {
  /// Additional fields for Mandate creation
  #[serde(skip_serializing_if = "Option::is_none")]
  mandate_options: Option<PaymentMethodOptionsAcssDebitMandateOptions>,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>,
  /// Bank account verification method.
  pub verification_method: String
}

/// If the PaymentIntent’s payment_method_types includes `affirm`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsAffirm {
  /// Controls when the funds will be captured from the customer’s account.
  pub capture_method: String,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `afterpay_clearpay`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsAfterpayClearpay {
  pub capture_method: String,
  /// Order identifier shown to the customer in Afterpay’s online portal.
  /// We recommend using a value that helps you answer any questions a customer might have about the payment.
  /// The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reference: Option<String>,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `alipay`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsAlipay {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `au_becs_debit`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsAuBecsDebit {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `bacs_debit`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsBacsDebit {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `bancontact`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsBancontact {
  /// Preferred language of the Bancontact authorization page that the customer is redirected to.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub preferred_language: Option<String>,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `blik`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsBlik;

/// If the PaymentIntent’s payment_method_types includes `boleto`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsBoleto {
  pub expires_after_days: u16,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// Installment plan selected for this PaymentIntent.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsCardInstallmentsPlan {
  /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
  pub count: u16,
  /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
  /// One of `month`.
  pub interval: String,
  /// Type of installment plan, one of `fixed_count`.
  pub r#type: String
}

/// Installment details for this payment (Mexico only).
/// 
/// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsCardInstallments {
  /// Installment plans that may be selected for this PaymentIntent.
  pub available_plans: Vec<PaymentMethodOptionsCardInstallmentsPlan>,
  /// Whether Installments are enabled for this PaymentIntent.
  pub enabled: bool,
  /// Installment plan selected for this PaymentIntent.
  pub plan: PaymentMethodOptionsCardInstallmentsPlan
}

/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsCardMandateOptions {
  /// Amount to be charged for future payments.
  pub amount: u32,
  /// One of `fixed` or `maximum`.
  /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
  /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
  pub amount_type: String,
  /// A description of the mandate or subscription that is meant to be displayed to the customer.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// End date of the mandate or subscription.
  /// If not provided, the mandate will be active until canceled.
  /// If provided, end date should be after start date.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end_date: Option<i64>,
  /// Specifies payment frequency.
  /// One of `day`, `week`, `month`, `year`, or `sporadic`.
  pub interval: String,
  /// The number of intervals between payments.
  /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
  /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
  /// This parameter is optional when `interval=sporadic`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub interval_count: Option<u16>,
  /// Unique identifier for the mandate or subscription.
  pub reference: String,
  /// Start date of the mandate or subscription.
  /// Start date should not be lesser than yesterday.
  pub start_date: i64,
  /// Specifies the type of mandates supported.
  /// Possible values are `india`.
  pub supported_types: Vec<String>
}

/// If the SetupIntent’s payment_method_types includes `card`, this hash contains the configurations that will be applied to each setup attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsCard {
  /// Controls when the funds will be captured from the customer’s account.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub capture_method: Option<String>,
  /// Installment details for this payment (Mexico only).
  /// 
  /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub installments: Option<PaymentMethodOptionsCardInstallments>,
  /// Configuration options for setting up an eMandate for cards issued in India.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mandate_options: Option<PaymentMethodOptionsCardMandateOptions>,
  /// Selected network to process this SetupIntent on.
  /// Depends on the available networks of the card attached to the setup intent.
  /// Can be only set confirm-time.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub network: Option<String>,
  /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
  /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
  /// Permitted values include: `automatic` or `any`.
  /// If not provided, defaults to `automatic`.
  /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
  pub request_three_d_secure: String,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>,
  /// Provides information about a card payment that customers see on their statements.
  /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
  /// Maximum 22 characters.
  /// On card statements, the concatenation of both prefix and suffix (including separators) will appear truncated to 22 characters.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub statement_descriptor_suffix_kana: Option<String>,
  /// Provides information about a card payment that customers see on their statements.
  /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
  /// Maximum 17 characters.
  /// On card statements, the concatenation of both prefix and suffix (including separators) will appear truncated to 17 characters.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub statement_descriptor_suffix_kanji: Option<String>,
}

/// If the PaymentIntent’s payment_method_types includes `card_present`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsCardPresent {
  /// Controls when the funds will be captured from the customer’s account.
  pub capture_method: String,
  /// Request ability to capture this payment beyond the standard [authorization validity window](https://stripe.com/docs/terminal/features/extended-authorizations#authorization-validity)
  pub request_extended_authorization: bool,
  /// Request ability to [increment](https://stripe.com/docs/terminal/features/incremental-authorizations) this PaymentIntent if the combination of MCC and card brand is eligible.
  /// Check [incremental_authorization_supported](https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported) in the [Confirm](https://stripe.com/docs/api/payment_intents/confirm) response to verify support.
  pub request_incremental_authorization_support: bool
}

/// Configuration for eu_bank_transfer
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
  /// The desired country code of the bank account information.
  /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
  pub country: String
}

/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsCustomerBalanceBankTransfer {
  /// Configuration for eu_bank_transfer
  #[serde(skip_serializing_if = "Option::is_none")]
  pub eu_bank_transfer: Option<PaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>
}

/// If the PaymentIntent’s payment_method_types includes `customer_balance`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsCustomerBalance {
  /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bank_tranfer: Option<PaymentMethodOptionsCustomerBalanceBankTransfer>,
  /// The funding method type to be used when there are not enough funds in the customer balance.
  /// Permitted values include: `bank_transfer`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub funding_type: Option<String>,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `eps`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsEps {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `fpx`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsFpx {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `giropay`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsGiropay {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `grabpay`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsGrabpay {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `ideal`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsIdeal {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `interac_present`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsInteracPresent;

/// If the PaymentIntent’s payment_method_types includes `klarna`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsKlarna {
  /// Controls when the funds will be captured from the customer’s account.
  pub capture_method: String
}

/// If the PaymentIntent’s payment_method_types includes `konbini`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsKonbini {
  /// An optional 10 to 11 digit numeric-only string determining the confirmation code at applicable convenience stores.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub confirmation_number: Option<String>,
  /// The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire.
  /// For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expires_after_days: Option<u16>,
  /// The timestamp at which the Konbini payment instructions will expire.
  /// Only one of `expires_after_days` or `expires_at` may be set.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expires_at: Option<i64>,
  /// A product descriptor of up to 22 characters, which will appear to customers at the convenience store.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub product_description: Option<String>,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the SetupIntent’s payment_method_types includes `link`, this hash contains the configurations that will be applied to each setup attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsLink {
  /// Controls when the funds will be captured from the customer’s account.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub capture_method: Option<String>,
  /// Token used for persistent Link logins.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub persistent_token: Option<String>,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `oxxo`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsOxxo {
  /// The number of calendar days before an OXXO invoice expires.
  /// For example, if you create an OXXO invoice on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
  pub expires_after_days: u16,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `p24`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsP24 {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `paynow`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsPaynow {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `pix`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsPix {
  /// The number of seconds (between 10 and 1209600) after which Pix payment will expire.
  pub expires_after_seconds: u32,
  /// The timestamp at which the Pix expires.
  pub expires_at: i64,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `promptpay`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsPromptpay {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// Additional fields for Mandate creation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsSepaDebitMandateOptions;

/// If the PaymentIntent’s `payment_method_types` includes sepa_debit, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsSepaDebit {
  /// Additional fields for Mandate creation
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mandata_options: Option<PaymentMethodOptionsSepaDebitMandateOptions>,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `sofort`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsSofort {
  /// Preferred language of the SOFORT authorization page that the customer is redirected to.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub preferred_language: Option<String>,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// Additional fields for Financial Connections Session creation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsUsBankAccountFinancialConnections {
  /// The list of permissions to request. The payment_method permission must be included.
  pub permissions: Vec<String>
}

/// If the PaymentIntent’s payment_method_types includes `us_bank_account`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsUsBankAccount {
  /// Additional fields for Financial Connections Session creation
  pub financial_connections: PaymentMethodOptionsUsBankAccountFinancialConnections,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `wechat_pay`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptionsWechatPay {
  /// The app ID registered with WeChat Pay. Only required when client is ios or android.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub app_id: Option<String>,
  /// The client type that the end customer will pay from
  pub client: String,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setup_future_usage: Option<String>
}

/// Payment-method-specific configuration for this SetupIntent.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodOptions {
  /// If the PaymentIntent’s payment_method_types includes `acss_debit`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub acss_debit: Option<PaymentMethodOptionsAcssDebit>,
  /// If the PaymentIntent’s payment_method_types includes `affirm`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub affirm: Option<PaymentMethodOptionsAffirm>,
  /// If the PaymentIntent’s payment_method_types includes `afterpay_clearpay`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub afterpay_clearpay: Option<PaymentMethodOptionsAfterpayClearpay>,
  /// If the PaymentIntent’s payment_method_types includes `alipay`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub alipay: Option<PaymentMethodOptionsAlipay>,
  /// If the PaymentIntent’s payment_method_types includes `au_becs_debit`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub au_becs_debit: Option<PaymentMethodOptionsAuBecsDebit>,
  /// If the PaymentIntent’s payment_method_types includes `bacs_debit`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bacs_debit: Option<PaymentMethodOptionsBacsDebit>,
  /// If the PaymentIntent’s payment_method_types includes `bancontact`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bancontact: Option<PaymentMethodOptionsBancontact>,
  /// If the PaymentIntent’s payment_method_types includes `blik`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blik: Option<PaymentMethodOptionsBlik>,
  /// If the PaymentIntent’s payment_method_types includes `boleto`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub boleto: Option<PaymentMethodOptionsBoleto>,
  /// If the SetupIntent’s payment_method_types includes `card`, this hash contains the configurations that will be applied to each setup attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub card: Option<PaymentMethodOptionsCard>,
  /// If the PaymentIntent’s payment_method_types includes `card_present`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub card_present: Option<PaymentMethodOptionsCardPresent>,
  /// If the PaymentIntent’s payment_method_types includes `customer_balance`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub customer_balance: Option<PaymentMethodOptionsCustomerBalance>,
  /// If the PaymentIntent’s payment_method_types includes `eps`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub eps: Option<PaymentMethodOptionsEps>,
  /// If the PaymentIntent’s payment_method_types includes `fpx`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fpx: Option<PaymentMethodOptionsFpx>,
  /// If the PaymentIntent’s payment_method_types includes `giropay`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub giropay: Option<PaymentMethodOptionsGiropay>,
  /// If the PaymentIntent’s payment_method_types includes `grabpay`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub grabpay: Option<PaymentMethodOptionsGrabpay>,
  /// If the PaymentIntent’s payment_method_types includes `ideal`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ideal: Option<PaymentMethodOptionsIdeal>,
  /// If the PaymentIntent’s payment_method_types includes `interac_present`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub interac_present: Option<PaymentMethodOptionsInteracPresent>,
  /// If the PaymentIntent’s payment_method_types includes `klarna`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub klarna: Option<PaymentMethodOptionsKlarna>,
  /// If the PaymentIntent’s payment_method_types includes `konbini`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub konbini: Option<PaymentMethodOptionsKonbini>,
  /// If the SetupIntent’s payment_method_types includes `link`, this hash contains the configurations that will be applied to each setup attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub link: Option<PaymentMethodOptionsLink>,
  /// If the PaymentIntent’s payment_method_types includes `oxxo`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub oxxo: Option<PaymentMethodOptionsOxxo>,
  /// If the PaymentIntent’s payment_method_types includes `p24`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub p24: Option<PaymentMethodOptionsP24>,
  /// If the PaymentIntent’s payment_method_types includes `paynow`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub paynow: Option<PaymentMethodOptionsPaynow>,
  /// If the PaymentIntent’s payment_method_types includes `pix`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pix: Option<PaymentMethodOptionsPix>,
  /// If the PaymentIntent’s payment_method_types includes `promptpay`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub promptpay: Option<PaymentMethodOptionsPromptpay>,
  /// If the PaymentIntent’s payment_method_types includes `sepa_debit`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sepa_debit: Option<PaymentMethodOptionsSepaDebit>,
  /// If the PaymentIntent’s payment_method_types includes `sofort`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sofort: Option<PaymentMethodOptionsSofort>,
  /// If the PaymentIntent’s payment_method_types includes `us_bank_account`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub us_bank_account: Option<PaymentMethodOptionsUsBankAccount>,
  /// If the PaymentIntent’s payment_method_types includes `wechat_pay`, this hash contains the configurations that will be applied to each payment attempt of that type.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wechat_pay: Option<PaymentMethodOptionsWechatPay>
}