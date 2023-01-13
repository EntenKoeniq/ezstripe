/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsAcssDebit;

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsAffirm;

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsAfterpayClearpay;

/// If the PaymentIntent’s payment_method_types includes `alipay`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsAlipay {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `au_becs_debit`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsAuBecsDebit {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `bacs_debit`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsBacsDebit {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>
}

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsBancontact;

/// If the PaymentIntent’s payment_method_types includes `blik`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsBlik;

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsBoleto;

/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsCardMandateOptions {
  /// Amount to be charged for future payments.
  pub amount: u32,
  /// One of `fixed` or `maximum`. If `fixed`, the `amount` param refers to the exact amount to be charged in future payments. If `maximum`, the amount charged can be up to the value passed for the `amount` param.
  pub amount_type: String,
  /// A description of the mandate or subscription that is meant to be displayed to the customer.
  pub description: Option<String>,
  /// End date of the mandate or subscription.
  /// If not provided, the mandate will be active until canceled.
  /// If provided, end date should be after start date.
  pub end_date: Option<i64>,
  /// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
  pub interval: String,
  /// The number of intervals between payments.
  /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
  /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
  /// This parameter is optional when `interval=sporadic`.
  pub interval_count: Option<String>,
  /// Unique identifier for the mandate or subscription.
  pub reference: String,
  /// Start date of the mandate or subscription.
  /// Start date should not be lesser than yesterday.
  pub start_date: i64,
  /// Specifies the type of mandates supported. Possible values are `india`.
  pub supported_types: Vec<String>
}

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsCardInstallments;

/// If the SetupIntent’s payment_method_types includes `card`, this hash contains the configurations that will be applied to each setup attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsCard {
  /// Controls when the funds will be captured from the customer’s account.
  pub capture_method: Option<String>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub installments: Option<PaymentMethodOptionsCardInstallments>,
  /// Configuration options for setting up an eMandate for cards issued in India.
  pub mandate_options: Option<PaymentMethodOptionsCardMandateOptions>,
  /// Selected network to process this SetupIntent on. Depends on the available networks of the card attached to the setup intent.
  /// Can be only set confirm-time.
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
  pub setup_future_usage: Option<String>,
  /// Provides information about a card payment that customers see on their statements.
  /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
  /// Maximum 22 characters. On card statements, the concatenation of both prefix and suffix (including separators) will appear truncated to 22 characters.
  pub statement_descriptor_suffix_kana: Option<String>,
  /// Provides information about a card payment that customers see on their statements. Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
  /// Maximum 17 characters.
  /// On card statements, the concatenation of both prefix and suffix (including separators) will appear truncated to 17 characters.
  pub statement_descriptor_suffix_kanji: Option<String>,
}

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsCardPresent;

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsCustomerBalance;

/// If the PaymentIntent’s payment_method_types includes `eps`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsEps {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `fpx`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsFpx {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `giropay`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsGiropay {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `grabpay`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsGrabpay {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `ideal`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsIdeal {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `interac_present`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsInteracPresent;

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsKlarna;

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsKombini;

/// If the SetupIntent’s payment_method_types includes `link`, this hash contains the configurations that will be applied to each setup attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsLink {
  /// Controls when the funds will be captured from the customer’s account.
  pub capture_method: Option<String>,
  /// Token used for persistent Link logins.
  pub persistent_token: Option<String>,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>
}

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsOxxo;

/// If the PaymentIntent’s payment_method_types includes `p24`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsP24 {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>
}

/// If the PaymentIntent’s payment_method_types includes `paynow`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsPaynow {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>
}

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsPix;

/// If the PaymentIntent’s payment_method_types includes `promptpay`, this hash contains the configurations that will be applied to each payment attempt of that type.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsPromptpay {
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>
}

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsSepaDebit;

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsSofort;

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsUsBankAccount;

/// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsWechatPay;

/// Payment-method-specific configuration for this SetupIntent.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptions {
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub acss_debit: Option<PaymentMethodOptionsAcssDebit>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub affirm: Option<PaymentMethodOptionsAffirm>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub afterpay_clearpay: Option<PaymentMethodOptionsAfterpayClearpay>,
  /// If the PaymentIntent’s payment_method_types includes `alipay`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub alipay: Option<PaymentMethodOptionsAlipay>,
  /// If the PaymentIntent’s payment_method_types includes `au_becs_debit`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub au_becs_debit: Option<PaymentMethodOptionsAuBecsDebit>,
  /// If the PaymentIntent’s payment_method_types includes `bacs_debit`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub bacs_debit: Option<PaymentMethodOptionsBacsDebit>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub bancontact: Option<PaymentMethodOptionsBancontact>,
  /// If the PaymentIntent’s payment_method_types includes `blik`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub blik: Option<PaymentMethodOptionsBlik>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub boleto: Option<PaymentMethodOptionsBoleto>,
  /// If the SetupIntent’s payment_method_types includes `card`, this hash contains the configurations that will be applied to each setup attempt of that type.
  pub card: Option<PaymentMethodOptionsCard>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub card_present: Option<PaymentMethodOptionsCardPresent>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub customer_balance: Option<PaymentMethodOptionsCustomerBalance>,
  /// If the PaymentIntent’s payment_method_types includes `eps`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub eps: Option<PaymentMethodOptionsEps>,
  /// If the PaymentIntent’s payment_method_types includes `fpx`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub fpx: Option<PaymentMethodOptionsFpx>,
  /// If the PaymentIntent’s payment_method_types includes `giropay`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub giropay: Option<PaymentMethodOptionsGiropay>,
  /// If the PaymentIntent’s payment_method_types includes `grabpay`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub grabpay: Option<PaymentMethodOptionsGrabpay>,
  /// If the PaymentIntent’s payment_method_types includes `ideal`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub ideal: Option<PaymentMethodOptionsIdeal>,
  /// If the PaymentIntent’s payment_method_types includes `interac_present`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub interac_present: Option<PaymentMethodOptionsInteracPresent>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub klarna: Option<PaymentMethodOptionsKlarna>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub kombini: Option<PaymentMethodOptionsKombini>,
  /// If the SetupIntent’s payment_method_types includes `link`, this hash contains the configurations that will be applied to each setup attempt of that type.
  pub link: Option<PaymentMethodOptionsLink>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub oxxo: Option<PaymentMethodOptionsOxxo>,
  /// If the PaymentIntent’s payment_method_types includes `p24`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub p24: Option<PaymentMethodOptionsP24>,
  /// If the PaymentIntent’s payment_method_types includes `paynow`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub paynow: Option<PaymentMethodOptionsPaynow>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub pix: Option<PaymentMethodOptionsPix>,
  /// If the PaymentIntent’s payment_method_types includes `promptpay`, this hash contains the configurations that will be applied to each payment attempt of that type.
  pub promptpay: Option<PaymentMethodOptionsPromptpay>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub sepa_debit: Option<PaymentMethodOptionsSepaDebit>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub sofort: Option<PaymentMethodOptionsSofort>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub us_bank_account: Option<PaymentMethodOptionsUsBankAccount>,
  /// **HELP US TO COMPLETE THE CONTENT ON [GITHUB](https://github.com/xEntenKoeniqx/ezstripe/pulls)**
  pub wechat_pay: Option<PaymentMethodOptionsWechatPay>
}