/// Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth).
/// The transactions are returned in sorted order, with the most recent transactions appearing first.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseList {
  /// String representing the object’s type.
  /// Objects of the same type share the same value.
  pub object: String,
  /// The end of the requested URL of the API.
  pub url: String,
  /// If more than the data received now exists `true` otherwise `false`.
  pub has_more: bool,
  /// All received data.
  pub data: Vec<Response>
}