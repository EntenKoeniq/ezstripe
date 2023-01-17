/// Returns a list of all refunds you’ve previously created.
/// The refunds are returned in sorted order, with the most recent refunds appearing first.
/// For convenience, the 10 most recent refunds are always available by default on the charge object.
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