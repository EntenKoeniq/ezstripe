/// Portion of the amount that corresponds to a tip.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AmountDetailsTip {
  /// Portion of the amount that corresponds to a tip.
  pub amount: Option<u32>
}

/// Details about items included in the amount
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AmountDetails {
  /// Portion of the amount that corresponds to a tip.
  pub tip: Option<AmountDetailsTip>
}