use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

/// Detailed breakdown of fees (in cents) paid for this transaction.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BalanceTransactionFeeDetails {
  /// Amount of the fee, in cents.
  pub amount: u32,
  /// ID of the Connect application that earned the fee.
  pub application: String,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
  /// Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// An arbitrary string attached to the object.
  /// Often useful for displaying to users.
  pub description: String,
  /// Type of the fee, one of: `application_fee`, `stripe_fee` or `tax`.
  pub r#type: String
}

/// Balance transactions represent funds moving through your Stripe account.
/// They're created for every type of transaction that comes into or flows out of your Stripe account balance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BalanceTransaction {
  /// Unique identifier for the object.
  pub id: String,
  /// String representing the object’s type. Objects of the same type share the same value.
  pub object: String, // balance_transaction
  /// Gross amount of the transaction, in cents.
  pub amount: u32,
  /// The date the transaction’s net funds will become available in the Stripe balance.
  pub available_on: i64,
  /// Time at which the object was created.
  /// Measured in seconds since the Unix epoch.
  pub created: i64,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
  /// Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// An arbitrary string attached to the object. Often useful for displaying to users.
  pub description: Option<String>,
  pub exchange_rate: Option<u32>,
  /// Fees (in cents) paid for this transaction.
  pub fee: u32,
  /// Detailed breakdown of fees (in cents) paid for this transaction.
  pub fee_details: Vec<BalanceTransactionFeeDetails>,
  /// Net amount of the transaction, in cents.
  pub net: u32,
  /// [Learn more](https://stripe.com/docs/reports/reporting-categories) about how reporting categories can help you understand balance transactions from an accounting perspective.
  pub reporting_category: String,
  /// The Stripe object to which this transaction is related.
  pub source: String,
  /// If the transaction’s net funds are available in the Stripe balance yet.
  /// Either `available` or `pending`.
  pub status: String,
  /// Transaction type: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
  /// [Learn more](https://stripe.com/docs/reports/balance-transaction-types) about balance transaction types and what they represent.
  /// If you are looking to classify transactions for accounting purposes, you might want to consider `reporting_category` instead.
  pub r#type: String
}

/// Information about the evidence submission.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EvidenceDetails {
  /// Date by which evidence must be submitted in order to successfully challenge dispute.
  /// Will be null if the customer’s bank or credit card company doesn’t allow a response for this particular dispute.
  pub due_by: i64,
  /// Whether evidence has been staged for this dispute.
  pub has_evidence: bool,
  /// Whether the last evidence submission was submitted past the due date.
  /// Defaults to `false` if no evidence submissions have occurred.
  /// If `true`, then delivery of the latest evidence is _not_ guaranteed.
  pub past_due: bool,
  /// The number of times evidence has been submitted. Typically, you may only submit evidence once.
  pub submission_count: u16
}

/// Evidence provided to respond to a dispute.
/// Updating any field in the hash will submit all fields in the hash for review.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Evidence {
  /// Any server or activity logs showing proof that the customer accessed or downloaded the purchased digital product.
  /// This information should include IP addresses, corresponding timestamps, and any detailed recorded activity.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub access_activity_log: Option<String>,
  /// The billing address provided by the customer.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub billing_address: Option<String>,
  /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your subscription cancellation policy, as shown to the customer.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cancellation_policy: Option<String>,
  /// An explanation of how and when the customer was shown your refund policy prior to purchase.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cancellation_policy_disclosure: Option<String>,
  /// A justification for why the customer’s subscription was not canceled.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cancellation_rebuttal: Option<String>,
  /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any communication with the customer that you feel is relevant to your case.
  /// Examples include emails proving that the customer received the product or service, or demonstrating their use of or satisfaction with the product or service.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub customer_communication: Option<String>,
  /// The email address of the customer.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub customer_email_address: Option<String>,
  /// The name of the customer.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub customer_name: Option<String>,
  /// The IP address that the customer used when making the purchase.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub customer_purchase_ip: Option<String>,
  /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A relevant document or contract showing the customer’s signature.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub customer_signature: Option<String>,
  /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation for the prior charge that can uniquely identify the charge, such as a receipt, shipping label, work order, etc. This document should be paired with a similar document from the disputed payment that proves the two payments are separate.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub duplicate_charge_documentation: Option<String>,
  /// An explanation of the difference between the disputed charge versus the prior charge that appears to be a duplicate.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub duplicate_charge_explanation: Option<String>,
  /// The Stripe ID for the prior charge which appears to be a duplicate of the disputed charge.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub duplicate_charge_id: Option<String>,
  /// A description of the product or service that was sold.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub product_description: Option<String>,
  /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload) Any receipt or message sent to the customer notifying them of the charge.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub receipt: Option<String>,
  /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload) Your refund policy, as shown to the customer.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub refund_policy: Option<String>,
  /// Documentation demonstrating that the customer was shown your refund policy prior to purchase.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub refund_policy_disclosure: Option<String>,
  /// A justification for why the customer is not entitled to a refund.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub refund_refusal_explanation: Option<String>,
  /// The date on which the customer received or began receiving the purchased service, in a clear human-readable format.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub service_date: Option<String>,
  /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload) Documentation showing proof that a service was provided to the customer.
  /// This could include a copy of a signed contract, work order, or other form of written agreement.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub service_documentation: Option<String>,
  /// The address to which a physical product was shipped.
  /// You should try to include as complete address information as possible.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub shipping_address: Option<String>,
  /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc. If multiple carriers were used for this purchase, please separate them with commas.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub shipping_carrier: Option<String>,
  /// The date on which a physical product began its route to the shipping address, in a clear human-readable format.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub shipping_date: Option<String>,
  /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload) Documentation showing proof that a product was shipped to the customer at the same address the customer provided to you.
  /// This could include a copy of the shipment receipt, shipping label, etc. It should show the customer’s full shipping address, if possible.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub shipping_documentation: Option<String>,
  /// The tracking number for a physical product, obtained from the delivery service.
  /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub shipping_tracking_number: Option<String>,
  /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload) Any additional evidence or statements.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub uncategorized_file: Option<String>,
  /// Any additional evidence or statements.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub uncategorized_text: Option<String>
}

/// Dispute object from 01/17/2023
/// 
/// [Dispute object](https://stripe.com/docs/api/disputes/object)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
  /// Unique identifier for the object.
  pub id: String,
  /// String representing the object’s type. Objects of the same type share the same value.
  pub object: String, // dispute
  /// Disputed amount. Usually the amount of the charge, but can differ (usually because of currency fluctuation or because only part of the order is disputed).
  pub amount: u32,
  /// List of zero, one, or two balance transactions that show funds withdrawn and reinstated to your Stripe account as a result of this dispute.
  pub balance_transactions: Vec<BalanceTransaction>,
  /// ID of the charge that was disputed.
  pub charge: String,
  /// Time at which the object was created. Measured in seconds since the Unix epoch.
  pub created: i64,
  /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
  /// Must be a [supported currency](https://stripe.com/docs/currencies).
  pub currency: String,
  /// Evidence provided to respond to a dispute.
  /// Updating any field in the hash will submit all fields in the hash for review.
  pub evidence: Evidence,
  /// Information about the evidence submission.
  pub evidence_details: EvidenceDetails,
  /// If true, it is still possible to refund the disputed payment. Once the payment has been fully refunded, no further funds will be withdrawn from your Stripe account as a result of this dispute.
  pub is_charge_refundable: bool,
  /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
  pub livemode: bool,
  /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
  /// This can be useful for storing additional information about the object in a structured format.
  pub metadata: HashMap<String, String>,
  /// ID of the PaymentIntent that was disputed.
  pub payment_intent: String,
  /// Reason given by cardholder for dispute.
  /// Possible values are `bank_cannot_process`, `check_returned`, `credit_not_processed`, `customer_initiated`, `debit_not_authorized`, `duplicate`, `fraudulent`, `general`, `incorrect_account_details`, `insufficient_funds`, `product_not_received`, `product_unacceptable`, `subscription_canceled`, or `unrecognized`. Read more about [dispute reasons](https://stripe.com/docs/disputes/categories).
  pub reason: String,
  /// Current status of dispute.
  /// Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `charge_refunded`, `won`, or `lost`.
  pub status: String
}

/// Returns a list of your disputes.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseList {
  /// String representing the object’s type.
  /// Objects of the same type share the same value.
  pub object: String,
  /// The end of the requested URL of the API.
  pub url: String,
  /// If more than the data received now exists `true` otherwise `false`.
  pub has_more: bool,
  /// All received data.
  pub data: Vec<Response>
}

#[doc(hidden)]
#[derive(PartialEq)]
pub enum Types {
  RETRIEVE(String),
  UPDATE(String, String),
  CLOSE(String),
  LIST(String)
}

#[doc(hidden)]
const DISPUTE_URL: &str = "https://api.stripe.com/v1/disputes";

#[doc(hidden)]
impl Types {
  pub fn create_send_request(&self, client: &reqwest::Client, secret: &str)-> reqwest::RequestBuilder {
    let mut result = client
      .post(self._get_url())
      .basic_auth(secret, None::<&str>)
      .header("Content-Type", "application/x-www-form-urlencoded");
    
    if let Some(r) = self._get_body() {
      result = result.body(r);
    }

    result
  }

  pub fn create_get_request(&self, client: &reqwest::Client, secret: &str)-> reqwest::RequestBuilder {
    let mut result = client
      .get(self._get_url())
      .basic_auth(secret, None::<&str>)
      .header("Content-Type", "application/x-www-form-urlencoded");
    
    if let Some(r) = self._get_body() {
      result = result.body(r);
    }

    result
  }

  fn _get_url(&self) -> String {
    match self {
      Self::RETRIEVE(id) => format!("{}/{}", DISPUTE_URL, id),
      Self::UPDATE(id, _) => format!("{}/{}", DISPUTE_URL, id),
      Self::CLOSE(id) => format!("{}/{}/close", DISPUTE_URL, id),
      Self::LIST(_) => format!("{}", DISPUTE_URL)
    }
  }

  fn _get_body(&self) -> Option<String> {
    let body = match self {
      Self::UPDATE(_, body) => body,
      Self::LIST(body) => body,
      _ => ""
    };

    if body.is_empty() {
      None
    } else {
      Some(body.to_string())
    }
  }
}

/// This structure contains all the data for a request to Stripe's API.
pub struct Info<'a> {
  /// The type of request to Stripe.
  pub r#type: Types,
  /// Stripe's API secret key.
  pub secret_key: String,
  // A reference to the `reqwest::Client` reusable.
  #[doc(hidden)]
  pub reqwest_client: &'a reqwest::Client
}

impl Info<'_> {
  /// Sends a "POST" request to Stripe's API.
  pub async fn send(&self) -> Result<Response, (String, Option<crate::error::Info>)> {
    match self.r#type {
      Types::RETRIEVE(_) => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `send()`. Please use the `get()` function");
        }
        return Err(("This function is not compatible with the selected type".to_string(), None));
      },
      Types::LIST(_) => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `send()`. Please use the `get_list()` function");
        }
        return Err(("This function is not compatible with the selected type".to_string(), None));
      },
      _ => ()
    };

    crate::helper::make_reqwest::<Response>(self.r#type.create_send_request(self.reqwest_client, &self.secret_key)).await
  }
  
  /// Sends a "GET" request to Stripe's API.
  pub async fn get(&self) -> Result<Response, (String, Option<crate::error::Info>)> {
    match self.r#type {
      Types::RETRIEVE(_) => (),
      Types::LIST(_) => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `get()`. Please use the `get_list()` function");
        }
        return Err(("This function is not compatible with the selected type".to_string(), None));
      },
      _ => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `get()`. Please use the `send()` function");
        }
        return Err(("This function is not compatible with the selected type".to_string(), None));
      }
    };
    
    crate::helper::make_reqwest::<Response>(self.r#type.create_get_request(self.reqwest_client, &self.secret_key)).await
  }

  /// Sends a "GET" request to Stripe's API.
  pub async fn get_list(&self) -> Result<ResponseList, (String, Option<crate::error::Info>)> {
    match self.r#type {
      Types::LIST(_) => (),
      Types::RETRIEVE(_) => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `get_list()`. Please use the `get()` function");
        }
        return Err(("This function is not compatible with the selected type".to_string(), None));
      },
      _ => {
        if log::log_enabled!(log::Level::Error) {
          log::error!("The selected type is not compatible with `get_list()`. Please use the `send()` function");
        }
        return Err(("This function is not compatible with the selected type".to_string(), None));
      }
    };
    
    crate::helper::make_reqwest::<ResponseList>(self.r#type.create_get_request(self.reqwest_client, &self.secret_key)).await
  }
}