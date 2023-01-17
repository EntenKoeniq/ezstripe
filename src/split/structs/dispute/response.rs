include!("balance_transaction.rs");

include!("evidence.rs");

/// Dispute object from 01/17/2023
/// 
/// [Dispute object](https://stripe.com/docs/api/disputes/object)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
  /// Unique identifier for the object.
  pub id: String,
  /// String representing the object’s type. Objects of the same type share the same value.
  pub object: String, // dispute
  /// Disputed amount. Usually the amount of the charge, but can differ (usually because of currency fluctuation or because only part of the order is disputed).
  pub amount: u32,
  /// List of zero, one, or two balance transactions that show funds withdrawn and reinstated to your Stripe account as a result of this dispute.
  pub balance_transactions: Vec<BalanceTransaction>,
  /// ID of the charge that was disputed.
  pub charge: String,
  /// Time at which the object was created. Measured in seconds since the Unix epoch.
  pub created: i64,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
  /// Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// Evidence provided to respond to a dispute.
  /// Updating any field in the hash will submit all fields in the hash for review.
  pub evidence: Evidence,
  /// Information about the evidence submission.
  pub evidence_details: EvidenceDetails,
  /// If true, it is still possible to refund the disputed payment. Once the payment has been fully refunded, no further funds will be withdrawn from your Stripe account as a result of this dispute.
  pub is_charge_refundable: bool,
  /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
  pub livemode: bool,
  /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
  /// This can be useful for storing additional information about the object in a structured format.
  pub metadata: HashMap<String, String>,
  /// ID of the PaymentIntent that was disputed.
  pub payment_intent: String,
  /// Reason given by cardholder for dispute.
  /// Possible values are `bank_cannot_process`, `check_returned`, `credit_not_processed`, `customer_initiated`, `debit_not_authorized`, `duplicate`, `fraudulent`, `general`, `incorrect_account_details`, `insufficient_funds`, `product_not_received`, `product_unacceptable`, `subscription_canceled`, or `unrecognized`. Read more about [dispute reasons](https://stripe.com/docs/disputes/categories).
  pub reason: String,
  /// Current status of dispute.
  /// Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `charge_refunded`, `won`, or `lost`.
  pub status: String
}