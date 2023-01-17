/// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerAcceptanceOffline;

/// If this is a Mandate accepted online, this hash contains details about the online acceptance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerAcceptanceOnline {
  /// The IP address from which the Mandate was accepted by the customer.
  pub ip_address: String,
  ///The user agent of the browser from which the Mandate was accepted by the customer.
  pub user_agent: String
}

/// Details about the customer’s acceptance of the mandate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerAcceptance {
  /// The time at which the customer accepted the Mandate.
  pub accepted_at: i64,
  /// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub offline: Option<CustomerAcceptanceOffline>,
  /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub online: Option<CustomerAcceptanceOnline>,
  /// The type of customer acceptance information included with the Mandate.
  /// One of `online` or `offline`.
  pub r#type: String
}