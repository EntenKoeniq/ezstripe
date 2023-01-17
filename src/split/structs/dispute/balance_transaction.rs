/// Detailed breakdown of fees (in cents) paid for this transaction.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BalanceTransactionFeeDetails {
  /// Amount of the fee, in cents.
  pub amount: u32,
  /// ID of the Connect application that earned the fee.
  pub application: String,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
  /// Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// An arbitrary string attached to the object.
  /// Often useful for displaying to users.
  pub description: String,
  /// Type of the fee, one of: `application_fee`, `stripe_fee` or `tax`.
  pub r#type: String
}

/// Balance transactions represent funds moving through your Stripe account.
/// They're created for every type of transaction that comes into or flows out of your Stripe account balance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BalanceTransaction {
  /// Unique identifier for the object.
  pub id: String,
  /// String representing the object’s type. Objects of the same type share the same value.
  pub object: String, // balance_transaction
  /// Gross amount of the transaction, in cents.
  pub amount: u32,
  /// The date the transaction’s net funds will become available in the Stripe balance.
  pub available_on: i64,
  /// Time at which the object was created.
  /// Measured in seconds since the Unix epoch.
  pub created: i64,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
  /// Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// An arbitrary string attached to the object. Often useful for displaying to users.
  pub description: Option<String>,
  pub exchange_rate: Option<u32>,
  /// Fees (in cents) paid for this transaction.
  pub fee: u32,
  /// Detailed breakdown of fees (in cents) paid for this transaction.
  pub fee_details: Vec<BalanceTransactionFeeDetails>,
  /// Net amount of the transaction, in cents.
  pub net: u32,
  /// [Learn more](https://stripe.com/docs/reports/reporting-categories) about how reporting categories can help you understand balance transactions from an accounting perspective.
  pub reporting_category: String,
  /// The Stripe object to which this transaction is related.
  pub source: String,
  /// If the transaction’s net funds are available in the Stripe balance yet.
  /// Either `available` or `pending`.
  pub status: String,
  /// Transaction type: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
  /// [Learn more](https://stripe.com/docs/reports/balance-transaction-types) about balance transaction types and what they represent.
  /// If you are looking to classify transactions for accounting purposes, you might want to consider `reporting_category` instead.
  pub r#type: String
}