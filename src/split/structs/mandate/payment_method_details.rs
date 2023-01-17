/// If this mandate is associated with a `acss_debit` payment method, this hash contains mandate information specific to the `acss_debit` payment method.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodDetailsAcssDebit {
  /// List of Stripe products where this mandate can be selected automatically.
  pub default_for: Vec<String>,
  /// Description of the interval.
  /// Only required if the ‘payment_schedule’ parameter is ‘interval’ or ‘combined’.
  pub interval_description: String,
  /// Payment schedule for the mandate.
  pub payment_schedule: String,
  /// Transaction type of the mandate.
  pub transaction_type: String
}

/// If this mandate is associated with a `au_becs_debit` payment method, this hash contains mandate information specific to the `au_becs_debit` payment method.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodDetailsAuBecsDebit {
  /// The URL of the mandate.
  /// This URL generally contains sensitive information about the customer and should be shared with them exclusively.
  pub url: String
}

/// If this mandate is associated with a `bacs_debit` payment method, this hash contains mandate information specific to the `bacs_debit` payment method.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodDetailsBacsDebit {
  /// The status of the mandate on the Bacs network.
  /// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
  pub network_status: String,
  /// The unique reference identifying the mandate on the Bacs network.
  pub reference: String,
  /// The URL that will contain the mandate that the customer has signed.
  pub url: String
}

/// Details for off-session mandates.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodDetailsBlikOffSession {
  /// Amount of each recurring payment.
  pub amount: u32,
  /// Currency of each recurring payment.
  pub currency: String,
  /// Frequency interval of each recurring payment.
  pub interval: String,
  /// Frequency indicator of each recurring payment.
  pub interval_count: u16
}

/// If this mandate is associated with a `blik` payment method, this hash contains mandate information specific to the `blik` payment method.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodDetailsBlik {
  /// Date at which the mandate expires.
  pub expires_after: i64,
  /// Details for off-session mandates.
  pub off_session: PaymentMethodDetailsBlikOffSession,
  /// Type of the mandate.
  pub r#type: String,
}

/// If this mandate is associated with a `card` payment method, this hash contains mandate information specific to the `card` payment method.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodDetailsCard;

/// If this mandate is associated with a `link` payment method, this hash contains mandate information specific to the `link` payment method.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodDetailsLink;

/// If this mandate is associated with a `sepa_debit` payment method, this hash contains mandate information specific to the `sepa_debit` payment method.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodDetailsSepaDebit {
  /// The unique reference of the mandate.
  pub reference: String,
  /// The URL of the mandate.
  /// This URL generally contains sensitive information about the customer and should be shared with them exclusively.
  pub url: String
}

/// If this mandate is associated with a `us_bank_account` payment method, this hash contains mandate information specific to the `us_bank_account` payment method.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodDetailsUsBankAccount;

/// Additional mandate information specific to the payment method type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethodDetails {
  /// If this mandate is associated with a `acss_debit` payment method, this hash contains mandate information specific to the `acss_debit` payment method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub acss_debit: Option<PaymentMethodDetailsAcssDebit>,
  /// If this mandate is associated with a `au_becs_debit` payment method, this hash contains mandate information specific to the `au_becs_debit` payment method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub au_becs_debit: Option<PaymentMethodDetailsAuBecsDebit>,
  /// If this mandate is associated with a `bacs_debit` payment method, this hash contains mandate information specific to the `bacs_debit` payment method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bacs_debit: Option<PaymentMethodDetailsBacsDebit>,
  /// If this mandate is associated with a `blik` payment method, this hash contains mandate information specific to the `blik` payment method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blik: Option<PaymentMethodDetailsBlik>,
  /// If this mandate is associated with a `card` payment method, this hash contains mandate information specific to the `card` payment method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub card: Option<PaymentMethodDetailsCard>,
  /// If this mandate is associated with a `link` payment method, this hash contains mandate information specific to the `link` payment method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub link: Option<PaymentMethodDetailsLink>,
  /// If this mandate is associated with a `sepa_debit` payment method, this hash contains mandate information specific to the `sepa_debit` payment method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sepa_debit: Option<PaymentMethodDetailsSepaDebit>,
  /// The type of the payment method associated with this mandate.
  /// An additional hash is included on `payment_method_details` with a name matching this value.
  /// It contains mandate information specific to the payment method.
  pub r#type: String,
  /// If this mandate is associated with a `us_bank_account` payment method, this hash contains mandate information specific to the `us_bank_account` payment method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub us_bank_account: Option<PaymentMethodDetailsUsBankAccount>
}