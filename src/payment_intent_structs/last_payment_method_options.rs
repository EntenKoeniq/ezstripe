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

/// If the SetupIntent’s payment_method_types includes `card`, this hash contains the configurations that will be applied to each setup attempt of that type.
/// 
/// MISSING DETAILS: `installments`, `network`, `setup_future_usage`, `statement_descriptor_suffix_kana` and `statement_descriptor_suffix_kanji`
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptionsCard {
  /// Controls when the funds will be captured from the customer’s account.
  pub capture_method: Option<String>,
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
  pub request_three_d_secure: Option<String>
}

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

/// Payment-method-specific configuration for this SetupIntent.
/// 
/// MISSING DETAILS: `acss_debit`, `blik`, `sepa_debit` and `us_bank_account`
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethodOptions {
  //pub acss_debit: ?,
  //pub blik: ?,
  /// If the SetupIntent’s payment_method_types includes `card`, this hash contains the configurations that will be applied to each setup attempt of that type.
  pub card: Option<PaymentMethodOptionsCard>,
  /// If the SetupIntent’s payment_method_types includes `link`, this hash contains the configurations that will be applied to each setup attempt of that type.
  pub link: Option<PaymentMethodOptionsLink>,
  //pub sepa_debit: ?,
  //pub us_bank_account: ?
}