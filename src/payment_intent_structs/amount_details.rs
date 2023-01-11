/// Portion of the amount that corresponds to a tip.
#[derive(Serialize, Deserialize, Debug)]
pub struct AmountDetailsTip {
  /// Portion of the amount that corresponds to a tip.
  pub amount: Option<u32>
}

/// Details about items included in the amount
#[derive(Serialize, Deserialize, Debug)]
pub struct AmountDetails {
  /// Portion of the amount that corresponds to a tip.
  pub tip: AmountDetailsTip
}