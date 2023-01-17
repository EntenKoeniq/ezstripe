/// Returns a list of PaymentIntents.
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