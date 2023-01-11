/// Shipping address.
#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingAddress {
  /// City, district, suburb, town, or village.
  pub city: Option<String>,
  /// Two-letter country code [(ISO 3166-1 alpha-2)](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
  pub country: Option<String>,
  /// Address line 1 (e.g., street, PO Box, or company name).
  pub line1: Option<String>,
  /// Address line 2 (e.g., apartment, suite, unit, or building).
  pub line2: Option<String>,
  /// ZIP or postal code.
  pub postal_code: Option<String>,
  /// State, county, province, or region.
  pub state: Option<String>
}

/// Shipping information for this PaymentIntent.
#[derive(Serialize, Deserialize, Debug)]
pub struct Shipping {
  /// Shipping address.
  pub address: ShippingAddress,
  /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
  pub carrier: Option<String>,
  /// Recipient name.
  pub name: String,
  /// Recipient phone (including extension).
  pub phone: Option<String>,
  /// The tracking number for a physical product, obtained from the delivery service.
  /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
  pub tracking_number: Option<String>
}