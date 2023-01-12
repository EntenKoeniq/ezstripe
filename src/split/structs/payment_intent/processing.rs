/// For recurring payments of Indian cards, this hash contains details on whether customer approval is required, and until when the payment will be in `processing` state
#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingCardCustomerNotification {
  /// Whether customer approval has been requested for this payment.
  /// For payments greater than INR 15000 or mandate amount, the customer must provide explicit approval of the payment with their bank.
  pub approval_requested: bool, // TODO: Check if `Option<>` required
  /// If customer approval is required, they need to provide approval before this time.
  pub completes_at: Option<i64> // TODO: Check if `Option<>` required
}

/// If the PaymentIntent’s payment_method_types includes `card`, this hash contains the details on the `processing` state of the payment.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingCard {
  /// For recurring payments of Indian cards, this hash contains details on whether customer approval is required, and until when the payment will be in `processing` state
  pub customer_notification: ProcessingCardCustomerNotification
}

/// If present, this property tells you about the processing state of the payment.
#[derive(Serialize, Deserialize, Debug)]
pub struct Processing {
  /// If the PaymentIntent’s payment_method_types includes `card`, this hash contains the details on the `processing` state of the payment.
  pub card: Option<ProcessingCard>,
  /// Type of the payment method for which payment is in `processing` state, one of `card`.
  pub r#type: String
}