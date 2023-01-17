/// Refunds object from 01/12/2023
/// 
/// [Refunds object](https://stripe.com/docs/api/refunds/object)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
  /// Unique identifier for the object.
  pub id: String,
  /// String representing the object’s type.
  /// Objects of the same type share the same value.
  pub object: String, // refund
  /// Amount, in cents.
  pub amount: u32,
  /// Balance transaction that describes the impact on your account balance.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub balance_transaction: Option<String>,
  /// ID of the charge that was refunded.
  pub charge: String,
  /// Time at which the object was created.
  /// Measured in seconds since the Unix epoch.
  pub created: i64,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
  /// Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
  /// This can be useful for storing additional information about the object in a structured format.
  pub metadata: HashMap<String, String>,
  /// ID of the PaymentIntent that was refunded.
  pub payment_intent: String,
  /// Reason for the refund, either user-provided (`duplicate`, `fraudulent`, or `requested_by_customer`) or generated by Stripe internally (`expired_uncaptured_charge`).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reason: Option<String>,
  /// This is the transaction number that appears on email receipts sent for this refund.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub receipt_number: Option<String>,
  /// The transfer reversal that is associated with the refund.
  /// Only present if the charge came from another Stripe account.
  /// See the Connect documentation for details.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source_transfer_reversal: Option<String>,
  /// Status of the refund.
  /// For credit card refunds, this can be `pending`, `succeeded`, or `failed`.
  /// For other types of refunds, it can be `pending`, `requires_action`, `succeeded`, `failed`, or `canceled`.
  /// Refer to our refunds documentation for more details.
  pub status: String,
  /// If the accompanying transfer was reversed, the transfer reversal object.
  /// Only applicable if the charge was created using the destination parameter.
  pub transfer_reversal: Option<String>
}