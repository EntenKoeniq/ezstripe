/// Payout object from 01/12/2023
/// 
/// [Payout object](https://stripe.com/docs/api/payouts/object)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
  /// Unique identifier for the object.
  pub id: String,
  /// String representing the object’s type.
  /// Objects of the same type share the same value.
  pub object: String, // payout
  /// Amount (in cents) to be transferred to your bank account or debit card.
  pub amount: u32,
  /// Date the payout is expected to arrive in the bank.
  /// This factors in delays like weekends or bank holidays.
  pub arrival_date: i64,
  /// Returns `true` if the payout was created by an [automated payout schedule](https://stripe.com/docs/payouts#payout-schedule), and `false` if it was [requested manually](https://stripe.com/docs/payouts#manual-payouts).
  pub automatic: bool,
  /// ID of the balance transaction that describes the impact of this payout on your account balance.
  pub balance_transaction: String,
  /// Time at which the object was created.
  /// Measured in seconds since the Unix epoch.
  pub created: i64,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
  /// Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// An arbitrary string attached to the object.
  /// Often useful for displaying to users.
  pub description: String,
  /// ID of the bank account or card the payout was sent to.
  pub destination: String,
  /// If the payout failed or was canceled, this will be the ID of the balance transaction that reversed the initial balance transaction, and puts the funds from the failed payout back in your balance.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub failure_balance_transaction: Option<String>,
  /// Error code explaining reason for payout failure if available.
  /// See [Types of payout failures](https://stripe.com/docs/api#payout_failures) for a list of failure codes.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub failure_code: Option<String>,
  /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
  pub livemode: bool,
  /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
  /// This can be useful for storing additional information about the object in a structured format.
  pub metadata: HashMap<String, String>,
  /// The method used to send this payout, which can be standard or instant.
  /// instant is only supported for payouts to debit cards. (See [Instant payouts for marketplaces](https://stripe.com/blog/instant-payouts-for-marketplaces) for more information.)
  pub method: String,
  /// If the payout reverses another, this is the ID of the original payout.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub original_payout: Option<String>,
  /// If the payout was reversed, this is the ID of the payout that reverses this payout.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reversed_by: Option<String>,
  /// The source balance this payout came from.
  /// One of `card`, `fpx`, or `bank_account`.
  pub source_type: String,
  /// Extra information about a payout to be displayed on the user’s bank statement.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub statement_descriptor: Option<String>,
  /// Current status of the payout: `paid`, `pending`, `in_transit`, `canceled` or `failed`.
  /// A payout is `pending` until it is submitted to the bank, when it becomes `in_transit`.
  /// The status then changes to paid if the transaction goes through, or to `failed` or `canceled` (within 5 business days).
  /// Some failed payouts may initially show as `paid` but then change to `failed`.
  pub status: String,
  /// Can be `bank_account` or `card`.
  pub r#type: String
}