include!("customer_acceptance.rs");

/// If this is a `multi_use` mandate, this hash contains details about the mandate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MultiUse;

include!("payment_method_details.rs");

/// If this is a `single_use` mandate, this hash contains details about the mandate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SingleUse {
  /// On a single use mandate, the amount of the payment.
  pub amount: u32,
  /// On a single use mandate, the currency of the payment.
  pub currency: String
}

/// Mandate object from 01/12/2023
/// 
/// [Mandate object](https://stripe.com/docs/api/mandates/object)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
  /// Unique identifier for the object.
  pub id: String,
  /// String representing the object’s type.
  /// Objects of the same type share the same value.
  pub object: String, // mandate
  /// Details about the customer’s acceptance of the mandate.
  pub customer_acceptance: CustomerAcceptance,
  /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
  pub livemode: bool,
  /// If this is a `multi_use` mandate, this hash contains details about the mandate.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub multi_use: Option<MultiUse>,
  /// ID of the payment method associated with this mandate.
  pub payment_method: String,
  /// Additional mandate information specific to the payment method type.
  pub payment_method_details: PaymentMethodDetails,
  /// The status of the mandate, which indicates whether it can be used to initiate a payment.
  pub status: String,
  /// The type of the mandate.
  pub r#type: String,
  /// If this is a `single_use` mandate, this hash contains details about the mandate.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub single_use: Option<String> // Possible
}