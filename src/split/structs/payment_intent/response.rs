include!("amount_details.rs");

include!("automatic_payment_methods.rs");

include!("last_payment_error.rs");

include!("next_action.rs");

include!("payment_method_options.rs");

include!("processing.rs");

include!("shipping.rs");

include!("transfer_data.rs");

/// Payment intent object from 01/08/2023
/// 
/// [Payment intent object](https://stripe.com/docs/api/payment_intents/object)
/// 
/// MISSING DETAILS: `next_action`
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  /// Unique identifier for the object.
  pub id: String,
  /// String representing the object’s type. Objects of the same type share the same value.
  pub object: String,
  /// Amount intended to be collected by this PaymentIntent.
  /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
  /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
  /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
  pub amount: u32,
  /// Amount that can be captured from this PaymentIntent.
  pub amount_capturable: u32,
  /// Details about items included in the amount
  pub amount_details: Option<AmountDetails>,
  /// Amount that was collected by this PaymentIntent.
  pub amount_received: u32,
  /// ID of the Connect application that created the PaymentIntent.
  pub application: Option<String>,
  /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner’s Stripe account.
  /// The amount of the application fee collected will be capped at the total payment amount.
  /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
  pub application_fee_amount: Option<u32>,
  /// Settings to configure compatible payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods)
  pub automatic_payment_methods: Option<AutomaticPaymentMethods>,
  /// Populated when `status` is `canceled`, this is the time at which the PaymentIntent was canceled.
  /// Measured in seconds since the Unix epoch.
  pub canceled_at: Option<i64>,
  /// Reason for cancellation of this PaymentIntent, either user-provided (`duplicate`, `fraudulent`, `requested_by_customer`, or `abandoned`) or generated by Stripe internally (`failed_invoice`, `void_invoice`, or `automatic`).
  pub cancellation_reason: Option<String>,
  /// Controls when the funds will be captured from the customer’s account.
  pub capture_method: String,
  /// The client secret of this PaymentIntent.
  /// Used for client-side retrieval using a publishable key.
  ///
  /// The client secret can be used to complete a payment from your frontend.
  /// It should not be stored, logged, or exposed to anyone other than the customer.
  /// Make sure that you have TLS enabled on any page that includes the client secret.
  /// 
  /// Refer to our docs to accept a payment and learn about how client_secret should be handled.
  pub client_secret: String,
  /// MISSING DOCUMENTATION
  pub confirmation_method: String,
  /// Time at which the object was created. Measured in seconds since the Unix epoch.
  pub created: i64,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// ID of the Customer this PaymentIntent belongs to, if one exists.
  ///
  /// Payment methods attached to other Customers cannot be used with this PaymentIntent.
  ///
  /// If present in combination with [setup_future_usage](https://stripe.com/docs/api/errors#payment_intent_object-setup_future_usage), this PaymentIntent’s payment method will be attached to the Customer after the PaymentIntent has been confirmed and any required actions from the user are complete.
  pub customer: Option<String>,
  /// An arbitrary string attached to the object. Often useful for displaying to users.
  pub description: Option<String>,
  /// ID of the invoice that created this PaymentIntent, if it exists.
  pub invoice: Option<String>,
  /// The payment error encountered in the previous PaymentIntent confirmation. It will be cleared if the PaymentIntent is later updated for any reason.
  pub last_payment_error: Option<LastPaymentError>,
  /// The latest charge created by this payment intent.
  pub latest_charge: Option<String>,
  /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
  pub livemode: bool,
  /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
  /// This can be useful for storing additional information about the object in a structured format.
  /// For more information, see the [documentation](https://stripe.com/docs/payments/payment-intents/creating-payment-intents#storing-information-in-metadata).
  pub metadata: HashMap<String, String>,
  pub next_action: Option<NextAction>,
  /// The account (if any) for which the funds of the PaymentIntent are intended.
  /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
  pub on_behalf_of: Option<String>,
  /// ID of the payment method used in this PaymentIntent.
  pub payment_method: Option<String>,
  /// Payment-method-specific configuration for this SetupIntent.
  /// 
  /// ONLY `card` AND `link` ARE CURRENTLY SUPPORTED!
  pub payment_method_options: Option<PaymentMethodOptions>,
  /// The list of payment method types (e.g. card) that this PaymentIntent is allowed to use.
  pub payment_method_types: Vec<String>,
  /// If present, this property tells you about the processing state of the payment.
  pub processing: Option<Processing>,
  /// Email address that the receipt for the resulting payment will be sent to.
  /// If `receipt_email` is specified for a payment in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
  pub receipt_email: Option<String>,
  /// ID of the review associated with this PaymentIntent, if any.
  pub review: Option<String>,
  /// Indicates that you intend to make future payments with this PaymentIntent’s payment method.
  /// 
  /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent’s Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
  /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
  /// 
  /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
  pub setup_future_usage: Option<String>,
  /// Shipping information for this PaymentIntent.
  pub shipping: Option<Shipping>,
  /// For non-card charges, you can use this value as the complete description that appears on your customers’ statements.
  /// Must contain at least one letter, maximum 22 characters.
  pub statement_descriptor: Option<String>,
  /// Provides information about a card payment that customers see on their statements.
  /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
  /// Maximum 22 characters for the concatenated descriptor.
  pub statement_descriptor_suffix: Option<String>,
  /// Status of this PaymentIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `requires_capture`, `canceled`, or `succeeded`.
  /// Read more about each PaymentIntent [status](https://stripe.com/docs/payments/intents#intent-statuses).
  pub status: String,
  /// The data with which to automatically create a Transfer when the payment is finalized.
  /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
  pub transfer_data: Option<TransferData>,
  /// A string that identifies the resulting payment as part of a group.
  /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
  pub transfer_group: Option<String>
}

impl Response {
  /// Get the complete Response as String.
  pub fn to_string(&self) -> Result<String, ()> {
    match serde_json::to_string(self) {
      Ok(r) => Ok(r),
      Err(_) => Err(())
    }
  }
}