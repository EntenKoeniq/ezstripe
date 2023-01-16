/// Portion of the amount that corresponds to a tip.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AmountDetailsTip {
  /// Portion of the amount that corresponds to a tip.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub amount: Option<u32>
}

/// Details about items included in the amount
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AmountDetails {
  /// Portion of the amount that corresponds to a tip.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tip: Option<AmountDetailsTip>
}