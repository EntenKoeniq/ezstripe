/// Shipping address.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShippingAddress {
  /// City, district, suburb, town, or village.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub city: Option<String>,
  /// Two-letter country code [(ISO 3166-1 alpha-2)](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub country: Option<String>,
  /// Address line 1 (e.g., street, PO Box, or company name).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub line1: Option<String>,
  /// Address line 2 (e.g., apartment, suite, unit, or building).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub line2: Option<String>,
  /// ZIP or postal code.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub postal_code: Option<String>,
  /// State, county, province, or region.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub state: Option<String>
}

/// Shipping information for this PaymentIntent.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Shipping {
  /// Shipping address.
  pub address: ShippingAddress,
  /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub carrier: Option<String>,
  /// Recipient name.
  pub name: String,
  /// Recipient phone (including extension).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phone: Option<String>,
  /// The tracking number for a physical product, obtained from the delivery service.
  /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tracking_number: Option<String>
}