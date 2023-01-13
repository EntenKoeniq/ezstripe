/// The payment error encountered in the previous PaymentIntent confirmation.
/// It will be cleared if the PaymentIntent is later updated for any reason.
#[derive(Serialize, Deserialize, Debug)]
pub struct LastPaymentError {
  /// For card errors, the ID of the failed charge.
  pub charge: Option<String>,
  /// For some errors that could be handled programmatically, a short string indicating the [error code](https://stripe.com/docs/error-codes) reported.
  pub code: String,
  /// For card errors resulting from a card issuer decline, a short string indicating the [card issuer’s reason for the decline](https://stripe.com/docs/declines#issuer-declines) if they provide one.
  pub decline_code: Option<String>,
  /// A URL to more information about the [error code](https://stripe.com/docs/error-codes) reported.
  pub doc_url: String,
  /// A human-readable message providing more details about the error.
  /// For card errors, these messages can be shown to your users.
  pub message: String,
  /// If the error is parameter-specific, the parameter related to the error.
  /// For example, you can use this to display a message near the correct form field.
  pub param: Option<String>,
  /// ID of the payment method used in this PaymentIntent.
  pub payment_method: Option<String>,
  /// If the error is specific to the type of payment method, the payment method type that had a problem.
  /// This field is only populated for invoice-related errors.
  pub payment_method_type: Option<String>,
  /// The type of error returned.
  /// One of `api_error`, `card_error`, `idempotency_error`, or `invalid_request_error`
  pub r#type: String
}