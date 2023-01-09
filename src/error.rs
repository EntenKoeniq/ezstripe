use serde::Serialize;

#[derive(Serialize)]
/// A list of possible HTTP errors.
pub enum HTTPCodes {
  /// Something went wrong.
  None,
  /// Everything worked as expected.
  Ok,
  /// The request was unacceptable, often due to missing a required parameter.
  BadRequest,
  /// No valid API key provided.
  Unauthorized,
  /// The parameters were valid but the request failed.
  RequestFailed,
  /// The API key doesn't have permissions to perform the request.
  Forbidden,
  /// The requested resource doesn't exist.
  NotFound,
  /// The request conflicts with another request (perhaps due to using the same idempotent key).
  Conflict,
  /// Too many requests hit the API too quickly. We recommend an exponential backoff of your requests.
  TooManyRequests,
  /// Something went wrong on Stripe's end. (These are rare.)
  ServerError(u16)
}

impl HTTPCodes {
  /// Get the correct enumeration by `input`.
  pub fn from_status(input: u16) -> Self {
    match input {
      200 => Self::Ok,
      400 => Self::BadRequest,
      401 => Self::Unauthorized,
      402 => Self::RequestFailed,
      403 => Self::Forbidden,
      404 => Self::NotFound,
      409 => Self::Conflict,
      429 => Self::TooManyRequests,
      500 | 502 | 503 | 504 => Self::ServerError(input),
      _ => Self::None
    }
  }

  /// Returns the original value.
  pub const fn original_u16(&self) -> u16 {
    match self {
      Self::Ok => 200,
      Self::BadRequest => 400,
      Self::Unauthorized => 401,
      Self::RequestFailed => 402,
      Self::Forbidden => 403,
      Self::NotFound => 404,
      Self::Conflict => 409,
      Self::TooManyRequests => 429,
      Self::ServerError(output) => *output,
      Self::None => 0
    }
  }
}

#[derive(Serialize)]
/// All available error codes from 01/08/2023
/// 
/// [Official Stripe error code list](https://stripe.com/docs/error-codes)
pub enum Codes {
  None,

  AccountCountryInvalidAddress,
  AccountErrorCountryChangeRequiresAdditionalSteps,
  AccountInformationMismatch,
  AccountInvalid,
  AccountNumberInvalid,

  AcssDebitSessionIncomplete,

  AlipayUpgradeRequired,

  AmountTooLarge,
  AmountTooSmall,

  ApiKeyExpired,

  AuthenticationRequired,

  BalanceInsufficient,

  BankAccountBadRoutingNumbers,
  BankAccountDeclined,
  BankAccountExists,
  BankAccountRestricted,
  BankAccountUnusable,
  BankAccountUnverified,
  BankAccountVerificationFailed,

  BillingInvalidMandate,

  BitcoinUpgradeRequired,

  CardDeclineRateLimitExceeded,
  CardDeclined,
  CardholderPhoneNumberRequired,

  ChargeAlreadyCaptured,
  ChargeAlreadyRefunded,
  ChargeDisputed,
  ChargeExceedsSourceLimit,
  ChargeExpiredForCapture,
  ChargeInvalidParamter,

  ClearingCodeUnsupported,

  CountryCodeInvalid,
  CountryUnsupported,

  CouponExpired,

  CustomerMaxPaymentMethods,
  CustomerMaxSubscriptions,

  DebitNotAuthorized,

  EmailInvalid,

  ExpiredCard,

  IdempotencyKeyInUse,

  IncorrectAddress,
  IncorrectCVC,
  IncorrectNumber,
  IncorrectZip,

  InstantPayoutsConfigDisabled,
  InstantPayoutsCurrencyDisabled,
  InstantPayoutsLimitExceeded,
  InstantPayoutsUnsupported,

  InsufficientFunds,

  IntentInvalidState,
  IntentVerificationMethodMissing,

  InvalidCardType,
  InvalidCharacters,
  InvalidChargeAmount,
  InvalidCVC,
  InvalidExpiryMonth,
  InvalidExpiryYear,
  InvalidNumber,
  InvalidSourceUsage,

  InvoiceNoCustomerLineItems,
  InvoiceNoPaymentMethodTypes,
  InvoiceNoSubscriptionLineItems,
  InvoiceNotEditable,
  InvoiceOnBehalfOfNotEditable,
  InvoicePaymentIntentRequiresAction,
  InvoiceUpcomingNone,

  LivemodeMismatch,

  LockTimeout,

  Missing,

  NoAccount,

  NotAllowedOnStandardAccount,

  OutOfInventory,

  OwnershipDeclarationNotAllowed,

  ParameterInvalidEmpty,
  ParameterInvalidInteger,
  ParameterInvalidStringBlank,
  ParameterInvalidStringEmpty,
  ParameterMissing,
  ParameterUnknown,
  
  ParametersExclusive,

  PaymentIntentActionRequired,
  PaymentIntentAuthenticationFailure,
  PaymentIntentIncompatiblePaymentMethod,
  PaymentIntentInvalidParameter,
  PaymentIntentKonbiniRejectedConfirmationNumber,
  PaymentIntentMandateInvalid,
  PaymentIntentPaymentAttemptExpired,
  PaymentIntentPaymentAttemptFailed,
  PaymentIntentUnexpectedState,
  
  PaymentMethodBankAccountAlreadyVerified,
  PaymentMethodBankAccountBlocked,
  PaymentMethodBillingDetailsAddressMissing,
  PaymentMethodCurrencyMismatch,
  PaymentMethodCustomerDecline,
  PaymentMethodInvalidParameter,
  PaymentMethodInvalidParameterTestmode,
  PaymentMethodMicrodepositFailed,
  PaymentMethodMicrodepositVerificationAmountsInvalid,
  PaymentMethodMicrodepositVerificationAmountsMismatch,
  PaymentMethodMicrodepositVerificationAttemptsExceeded,
  PaymentMethodMicrodepositVerificationDescriptorCodeMismatch,
  PaymentMethodMicrodepositVerificationTimeout,
  PaymentMethodProviderDecline,
  PaymentMethodProviderTimeout,
  PaymentMethodUnactivated,
  PaymentMethodUnexpectedState,
  PaymentMethodUnsupportedType,

  PayoutsNotAllowed,

  PlatformAccountRequired,
  PlatformApiKeyExpired,

  PostalCodeInvalid,

  ProcessingError,

  ProductInactive,

  RateLimit,

  ReferToCustomer,

  RefundDisputedPayment,

  ResourceAlreadyExists,
  ResourceMissing,

  ReturnIntentAlreadyProcessed,

  RoutingNumberInvalid,

  SecretKeyRequired,

  SepaUnsupportedAccount,

  SetupAttemptFailed,
  SetupIntentAuthenticationFailure,
  SetupIntentInvalidParameter,
  SetupIntentSetupAttemptExpired,
  SetupIntentUnexpectedState,

  ShippingCalculationFailed,

  SkuInactive,

  StateUnsupported,

  StatusTransitionInvalid,

  TaxIdInvalid,

  TaxesCalculationFailed,

  TerminalLocationCountryUnsupported,

  TestmodeChargesOnly,

  TlsVersionUnsupported,

  TokenAlreadyUsed,
  TokenInUse,

  TransferSourceBalanceParametersMismatch,

  TransfersNotAllowed,

  UrlInvalid
}

impl Codes {
  /// Get the correct enumeration by `input`.
  pub fn from_str(input: &str) -> Self {
    match input {
      "account_country_invalid_address" => Self::AccountCountryInvalidAddress,
      "account_error_country_change_requires_additional_steps" => Self::AccountErrorCountryChangeRequiresAdditionalSteps,
      "account_information_mismatch" => Self::AccountInformationMismatch,
      "account_invalid" => Self::AccountInvalid,
      "account_number_invalid" => Self::AccountNumberInvalid,
      "acss_debit_session_incomplete" => Self::AcssDebitSessionIncomplete,
      "alipay_upgrade_required" => Self::AlipayUpgradeRequired,
      "amount_too_large" => Self::AmountTooLarge,
      "amount_too_small" => Self::AmountTooSmall,
      "api_key_expired" => Self::ApiKeyExpired,
      "authentication_required" => Self::AuthenticationRequired,
      "balance_insufficient" => Self::BalanceInsufficient,
      "bank_account_bad_routing_numbers" => Self::BankAccountBadRoutingNumbers,
      "bank_account_declined" => Self::BankAccountDeclined,
      "bank_account_exists" => Self::BankAccountExists,
      "bank_account_restricted" => Self::BankAccountRestricted,
      "bank_account_unusable" => Self::BankAccountUnusable,
      "bank_account_unverified" => Self::BankAccountUnverified,
      "bank_account_verification_failed" => Self::BankAccountVerificationFailed,
      "billing_invalid_mandate" => Self::BillingInvalidMandate,
      "bitcoin_upgrade_required" => Self::BitcoinUpgradeRequired,
      "card_decline_rate_limit_exceeded" => Self::CardDeclineRateLimitExceeded,
      "card_declined" => Self::CardDeclined,
      "cardholder_phone_number_required" => Self::CardholderPhoneNumberRequired,
      "charge_already_captured" => Self::ChargeAlreadyCaptured,
      "charge_already_refunded" => Self::ChargeAlreadyRefunded,
      "charge_disputed" => Self::ChargeDisputed,
      "charge_exceeds_source_limit" => Self::ChargeExceedsSourceLimit,
      "charge_expired_for_capture" => Self::ChargeExpiredForCapture,
      "charge_invalid_parameter" => Self::ChargeInvalidParamter,
      "clearing_code_unsupported" => Self::ClearingCodeUnsupported,
      "country_code_invalid" => Self::CountryCodeInvalid,
      "country_unsupported" => Self::CountryUnsupported,
      "coupon_expired" => Self::CouponExpired,
      "customer_max_payment_methods" => Self::CustomerMaxPaymentMethods,
      "customer_max_subscriptions" => Self::CustomerMaxSubscriptions,
      "debit_not_authorized" => Self::DebitNotAuthorized,
      "email_invalid" => Self::EmailInvalid,
      "expired_card" => Self::ExpiredCard,
      "idempotency_key_in_use" => Self::IdempotencyKeyInUse,
      "incorrect_address" => Self::IncorrectAddress,
      "incorrect_cvc" => Self::IncorrectCVC,
      "incorrect_number" => Self::IncorrectNumber,
      "incorrect_zip" => Self::IncorrectZip,
      "instant_payouts_config_disabled" => Self::InstantPayoutsConfigDisabled,
      "instant_payouts_currency_disabled" => Self::InstantPayoutsCurrencyDisabled,
      "instant_payouts_limit_exceeded" => Self::InstantPayoutsLimitExceeded,
      "instant_payouts_unsupported" => Self::InstantPayoutsUnsupported,
      "insufficient_funds" => Self::InsufficientFunds,
      "intent_invalid_state" => Self::IntentInvalidState,
      "intent_verification_method_missing" => Self::IntentVerificationMethodMissing,
      "invalid_card_type" => Self::InvalidCardType,
      "invalid_characters" => Self::InvalidCharacters,
      "invalid_charge_amount" => Self::InvalidChargeAmount,
      "invalid_cvc" => Self::InvalidCVC,
      "invalid_expiry_month" => Self::InvalidExpiryMonth,
      "invalid_expiry_year" => Self::InvalidExpiryYear,
      "invalid_number" => Self::InvalidNumber,
      "invalid_source_usage" => Self::InvalidSourceUsage,
      "invoice_no_customer_line_items" => Self::InvoiceNoCustomerLineItems,
      "invoice_no_payment_method_types" => Self::InvoiceNoPaymentMethodTypes,
      "invoice_no_subscription_line_items" => Self::InvoiceNoSubscriptionLineItems,
      "invoice_not_editable" => Self::InvoiceNotEditable,
      "invoice_on_behalf_of_not_editable" => Self::InvoiceOnBehalfOfNotEditable,
      "invoice_payment_intent_requires_action" => Self::InvoicePaymentIntentRequiresAction,
      "invoice_upcoming_none" => Self::InvoiceUpcomingNone,
      "livemode_mismatch" => Self::LivemodeMismatch,
      "lock_timeout" => Self::LockTimeout,
      "missing" => Self::Missing,
      "no_account" => Self::NoAccount,
      "not_allowed_on_standard_account" => Self::NotAllowedOnStandardAccount,
      "out_of_inventory" => Self::OutOfInventory,
      "ownership_declaration_not_allowed" => Self::OwnershipDeclarationNotAllowed,
      "parameter_invalid_empty" => Self::ParameterInvalidEmpty,
      "parameter_invalid_integer" => Self::ParameterInvalidInteger,
      "parameter_invalid_string_blank" => Self::ParameterInvalidStringBlank,
      "parameter_invalid_string_empty" => Self::ParameterInvalidStringEmpty,
      "parameter_missing" => Self::ParameterMissing,
      "paremeter_unknown" => Self::ParameterUnknown,
      "parameters_exclusive" => Self::ParametersExclusive,
      "payment_intent_action_required" => Self::PaymentIntentActionRequired,
      "payment_intent_authentication_required" => Self::PaymentIntentAuthenticationFailure,
      "payment_intent_incompatible_payment_method" => Self::PaymentIntentIncompatiblePaymentMethod,
      "payment_intent_invalid_parameter" => Self::PaymentIntentInvalidParameter,
      "payment_intent_konbini_rejected_confirmation_number" => Self::PaymentIntentKonbiniRejectedConfirmationNumber,
      "payment_intent_mandate_invalid" => Self::PaymentIntentMandateInvalid,
      "payment_intent_payment_attempt_expired" => Self::PaymentIntentPaymentAttemptExpired,
      "payment_intent_payment_attempt_failed" => Self::PaymentIntentPaymentAttemptFailed,
      "payment_intent_unexpected_state" => Self::PaymentIntentUnexpectedState,
      "payment_method_bank_account_already_verified" => Self::PaymentMethodBankAccountAlreadyVerified,
      "payment_method_bank_account_blocked" => Self::PaymentMethodBankAccountBlocked,
      "payment_method_billing_details_address_missing" => Self::PaymentMethodBillingDetailsAddressMissing,
      "payment_method_currency_mismatch" => Self::PaymentMethodCurrencyMismatch,
      "payment_method_customer_decline" => Self::PaymentMethodCustomerDecline,
      "payment_method_invalid_parameter" => Self::PaymentMethodInvalidParameter,
      "payment_method_invalid_parameter_testmode" => Self::PaymentMethodInvalidParameterTestmode,
      "payment_method_microdeposit_failed" => Self::PaymentMethodMicrodepositFailed,
      "payment_method_microdeposit_verification_amounts_invalid" => Self::PaymentMethodMicrodepositVerificationAmountsInvalid,
      "payment_method_microdeposit_verification_amounts_mismatch" => Self::PaymentMethodMicrodepositVerificationAmountsMismatch,
      "payment_method_microdeposit_verification_attempts_exceeded" => Self::PaymentMethodMicrodepositVerificationAttemptsExceeded,
      "payment_method_microdeposit_verification_descriptor_code_mismatch" => Self::PaymentMethodMicrodepositVerificationDescriptorCodeMismatch,
      "payment_method_microdeposit_verification_timeout" => Self::PaymentMethodMicrodepositVerificationTimeout,
      "payment_method_provider_decline" => Self::PaymentMethodProviderDecline,
      "payment_method_provider_timeout" => Self::PaymentMethodProviderTimeout,
      "payment_method_unactivated" => Self::PaymentMethodUnactivated,
      "payment_method_unexpected_state" => Self::PaymentMethodUnexpectedState,
      "payment_method_unsupported_type" => Self::PaymentMethodUnsupportedType,
      "payouts_not_allowed" => Self::PayoutsNotAllowed,
      "platform_account_required" => Self::PlatformAccountRequired,
      "platform_api_key_expired" => Self::PlatformApiKeyExpired,
      "postal_code_invalid" => Self::PostalCodeInvalid,
      "processing_error" => Self::ProcessingError,
      "product_inactive" => Self::ProductInactive,
      "rate_limit" => Self::RateLimit,
      "refer_to_customer" => Self::ReferToCustomer,
      "refund_disputed_payment" => Self::RefundDisputedPayment,
      "resource_already_exists" => Self::ResourceAlreadyExists,
      "resource_missing" => Self::ResourceMissing,
      "return_intent_already_processed" => Self::ReturnIntentAlreadyProcessed,
      "routing_number_invalid" => Self::RoutingNumberInvalid,
      "secret_key_required" => Self::SecretKeyRequired,
      "sepa_unsupported_account" => Self::SepaUnsupportedAccount,
      "setup_attempt_failed" => Self::SetupAttemptFailed,
      "setup_intent_authentication_failure" => Self::SetupIntentAuthenticationFailure,
      "setup_intent_invalid_parameter" => Self::SetupIntentInvalidParameter,
      "setup_intent_setup_attempt_expired" => Self::SetupIntentSetupAttemptExpired,
      "setup_intent_unexpected_state" => Self::SetupIntentUnexpectedState,
      "shipping_calculation_failed" => Self::ShippingCalculationFailed,
      "sku_inactive" => Self::SkuInactive,
      "state_unsupported" => Self::StateUnsupported,
      "status_transition_invalid" => Self::StatusTransitionInvalid,
      "tax_id_invalid" => Self::TaxIdInvalid,
      "taxes_calculation_failed" => Self::TaxesCalculationFailed,
      "terminal_location_country_unsupported" => Self::TerminalLocationCountryUnsupported,
      "testmode_charges_only" => Self::TestmodeChargesOnly,
      "tls_version_unsupported" => Self::TlsVersionUnsupported,
      "token_already_used" => Self::TokenAlreadyUsed,
      "token_in_use" => Self::TokenInUse,
      "transfer_source_balance_parameters_mismatch" => Self::TransferSourceBalanceParametersMismatch,
      "transfers_not_allowed" => Self::TransfersNotAllowed,
      "url_invalid" => Self::UrlInvalid,
      _ => Self::None
    }
  }

  /// Returns the original value.
  pub const fn original_str(&self) -> &'static str {    
    match self {
      Self::AccountCountryInvalidAddress => "account_country_invalid_address",
      Self::AccountErrorCountryChangeRequiresAdditionalSteps => "account_error_country_change_requires_additional_steps",
      Self::AccountInformationMismatch => "account_information_mismatch",
      Self::AccountInvalid => "account_invalid",
      Self::AccountNumberInvalid => "account_number_invalid",
      Self::AcssDebitSessionIncomplete => "acss_debit_session_incomplete",
      Self::AlipayUpgradeRequired => "alipay_upgrade_required",
      Self::AmountTooLarge => "amount_too_large",
      Self::AmountTooSmall => "amount_too_small",
      Self::ApiKeyExpired => "api_key_expired",
      Self::AuthenticationRequired => "authentication_required",
      Self::BalanceInsufficient => "balance_insufficient",
      Self::BankAccountBadRoutingNumbers => "bank_account_bad_routing_numbers",
      Self::BankAccountDeclined => "bank_account_declined",
      Self::BankAccountExists => "bank_account_exists",
      Self::BankAccountRestricted => "bank_account_restricted",
      Self::BankAccountUnusable => "bank_account_unusable",
      Self::BankAccountUnverified => "bank_account_unverified",
      Self::BankAccountVerificationFailed => "bank_account_verification_failed",
      Self::BillingInvalidMandate => "billing_invalid_mandate",
      Self::BitcoinUpgradeRequired => "bitcoin_upgrade_required",
      Self::CardDeclineRateLimitExceeded => "card_decline_rate_limit_exceeded",
      Self::CardDeclined => "card_declined",
      Self::CardholderPhoneNumberRequired => "cardholder_phone_number_required",
      Self::ChargeAlreadyCaptured => "charge_already_captured",
      Self::ChargeAlreadyRefunded => "charge_already_refunded",
      Self::ChargeDisputed => "charge_disputed",
      Self::ChargeExceedsSourceLimit => "charge_exceeds_source_limit",
      Self::ChargeExpiredForCapture => "charge_expired_for_capture",
      Self::ChargeInvalidParamter => "charge_invalid_parameter",
      Self::ClearingCodeUnsupported => "clearing_code_unsupported",
      Self::CountryCodeInvalid => "country_code_invalid",
      Self::CountryUnsupported => "country_unsupported",
      Self::CouponExpired => "coupon_expired",
      Self::CustomerMaxPaymentMethods => "customer_max_payment_methods",
      Self::CustomerMaxSubscriptions => "customer_max_subscriptions",
      Self::DebitNotAuthorized => "debit_not_authorized",
      Self::EmailInvalid => "email_invalid",
      Self::ExpiredCard => "expired_card",
      Self::IdempotencyKeyInUse => "idempotency_key_in_use",
      Self::IncorrectAddress => "incorrect_address",
      Self::IncorrectCVC => "incorrect_cvc",
      Self::IncorrectNumber => "incorrect_number",
      Self::IncorrectZip => "incorrect_zip",
      Self::InstantPayoutsConfigDisabled => "instant_payouts_config_disabled",
      Self::InstantPayoutsCurrencyDisabled => "instant_payouts_currency_disabled",
      Self::InstantPayoutsLimitExceeded => "instant_payouts_limit_exceeded",
      Self::InstantPayoutsUnsupported => "instant_payouts_unsupported",
      Self::InsufficientFunds => "insufficient_funds",
      Self::IntentInvalidState => "intent_invalid_state",
      Self::IntentVerificationMethodMissing => "intent_verification_method_missing",
      Self::InvalidCardType => "invalid_card_type",
      Self::InvalidCharacters => "invalid_characters",
      Self::InvalidChargeAmount => "invalid_charge_amount",
      Self::InvalidCVC => "invalid_cvc",
      Self::InvalidExpiryMonth => "invalid_expiry_month",
      Self::InvalidExpiryYear => "invalid_expiry_year",
      Self::InvalidNumber => "invalid_number",
      Self::InvalidSourceUsage => "invalid_source_usage",
      Self::InvoiceNoCustomerLineItems => "invoice_no_customer_line_items",
      Self::InvoiceNoPaymentMethodTypes => "invoice_no_payment_method_types",
      Self::InvoiceNoSubscriptionLineItems => "invoice_no_subscription_line_items",
      Self::InvoiceNotEditable => "invoice_not_editable",
      Self::InvoiceOnBehalfOfNotEditable => "invoice_on_behalf_of_not_editable",
      Self::InvoicePaymentIntentRequiresAction => "invoice_payment_intent_requires_action",
      Self::InvoiceUpcomingNone => "invoice_upcoming_none",
      Self::LivemodeMismatch => "livemode_mismatch",
      Self::LockTimeout => "lock_timeout",
      Self::Missing => "missing",
      Self::NoAccount => "no_account",
      Self::NotAllowedOnStandardAccount => "not_allowed_on_standard_account",
      Self::OutOfInventory => "out_of_inventory",
      Self::OwnershipDeclarationNotAllowed => "ownership_declaration_not_allowed",
      Self::ParameterInvalidEmpty => "parameter_invalid_empty",
      Self::ParameterInvalidInteger => "parameter_invalid_integer",
      Self::ParameterInvalidStringBlank => "parameter_invalid_string_blank",
      Self::ParameterInvalidStringEmpty => "parameter_invalid_string_empty",
      Self::ParameterMissing => "parameter_missing",
      Self::ParameterUnknown => "paremeter_unknown",
      Self::ParametersExclusive => "parameters_exclusive",
      Self::PaymentIntentActionRequired => "payment_intent_action_required",
      Self::PaymentIntentAuthenticationFailure => "payment_intent_authentication_required",
      Self::PaymentIntentIncompatiblePaymentMethod => "payment_intent_incompatible_payment_method",
      Self::PaymentIntentInvalidParameter => "payment_intent_invalid_parameter",
      Self::PaymentIntentKonbiniRejectedConfirmationNumber => "payment_intent_konbini_rejected_confirmation_number",
      Self::PaymentIntentMandateInvalid => "payment_intent_mandate_invalid",
      Self::PaymentIntentPaymentAttemptExpired => "payment_intent_payment_attempt_expired",
      Self::PaymentIntentPaymentAttemptFailed => "payment_intent_payment_attempt_failed",
      Self::PaymentIntentUnexpectedState => "payment_intent_unexpected_state",
      Self::PaymentMethodBankAccountAlreadyVerified => "payment_method_bank_account_already_verified",
      Self::PaymentMethodBankAccountBlocked => "payment_method_bank_account_blocked",
      Self::PaymentMethodBillingDetailsAddressMissing => "payment_method_billing_details_address_missing",
      Self::PaymentMethodCurrencyMismatch => "payment_method_currency_mismatch",
      Self::PaymentMethodCustomerDecline => "payment_method_customer_decline",
      Self::PaymentMethodInvalidParameter => "payment_method_invalid_parameter",
      Self::PaymentMethodInvalidParameterTestmode => "payment_method_invalid_parameter_testmode",
      Self::PaymentMethodMicrodepositFailed => "payment_method_microdeposit_failed",
      Self::PaymentMethodMicrodepositVerificationAmountsInvalid => "payment_method_microdeposit_verification_amounts_invalid",
      Self::PaymentMethodMicrodepositVerificationAmountsMismatch => "payment_method_microdeposit_verification_amounts_mismatch",
      Self::PaymentMethodMicrodepositVerificationAttemptsExceeded => "payment_method_microdeposit_verification_attempts_exceeded",
      Self::PaymentMethodMicrodepositVerificationDescriptorCodeMismatch => "payment_method_microdeposit_verification_descriptor_code_mismatch",
      Self::PaymentMethodMicrodepositVerificationTimeout => "payment_method_microdeposit_verification_timeout",
      Self::PaymentMethodProviderDecline => "payment_method_provider_decline",
      Self::PaymentMethodProviderTimeout => "payment_method_provider_timeout",
      Self::PaymentMethodUnactivated => "payment_method_unactivated",
      Self::PaymentMethodUnexpectedState => "payment_method_unexpected_state",
      Self::PaymentMethodUnsupportedType => "payment_method_unsupported_type",
      Self::PayoutsNotAllowed => "payouts_not_allowed",
      Self::PlatformAccountRequired => "platform_account_required",
      Self::PlatformApiKeyExpired => "platform_api_key_expired",
      Self::PostalCodeInvalid => "postal_code_invalid",
      Self::ProcessingError => "processing_error",
      Self::ProductInactive => "product_inactive",
      Self::RateLimit => "rate_limit",
      Self::ReferToCustomer => "refer_to_customer",
      Self::RefundDisputedPayment => "refund_disputed_payment",
      Self::ResourceAlreadyExists => "resource_already_exists",
      Self::ResourceMissing => "resource_missing",
      Self::ReturnIntentAlreadyProcessed => "return_intent_already_processed",
      Self::RoutingNumberInvalid => "routing_number_invalid",
      Self::SecretKeyRequired => "secret_key_required",
      Self::SepaUnsupportedAccount => "sepa_unsupported_account",
      Self::SetupAttemptFailed => "setup_attempt_failed",
      Self::SetupIntentAuthenticationFailure => "setup_intent_authentication_failure",
      Self::SetupIntentInvalidParameter => "setup_intent_invalid_parameter",
      Self::SetupIntentSetupAttemptExpired => "setup_intent_setup_attempt_expired",
      Self::SetupIntentUnexpectedState => "setup_intent_unexpected_state",
      Self::ShippingCalculationFailed => "shipping_calculation_failed",
      Self::SkuInactive => "sku_inactive",
      Self::StateUnsupported => "state_unsupported",
      Self::StatusTransitionInvalid => "status_transition_invalid",
      Self::TaxIdInvalid => "tax_id_invalid",
      Self::TaxesCalculationFailed => "taxes_calculation_failed",
      Self::TerminalLocationCountryUnsupported => "terminal_location_country_unsupported",
      Self::TestmodeChargesOnly => "testmode_charges_only",
      Self::TlsVersionUnsupported => "tls_version_unsupported",
      Self::TokenAlreadyUsed => "token_already_used",
      Self::TokenInUse => "token_in_use",
      Self::TransferSourceBalanceParametersMismatch => "transfer_source_balance_parameters_mismatch",
      Self::TransfersNotAllowed => "transfers_not_allowed",
      Self::UrlInvalid => "url_invalid",
      Self::None => ""
    }
  }
}

#[derive(Serialize)]
pub enum Types {
  /// ?
  None,
  /// API errors cover any other type of problem (e.g., a temporary problem with Stripe's servers), and are extremely uncommon.
  API,
  /// Card errors are the most common type of error you should expect to handle. They result when the user enters a card that can't be charged for some reason.
  Card,
  /// Idempotency errors occur when an Idempotency-Key is re-used on a request that does not match the first request's API endpoint and parameters.
  Idempotency,
  /// Invalid request errors arise when your request has invalid parameters.
  InvalidRequest
}

impl Types {
  /// Get the correct enumeration by `input`.
  pub fn from_str(input: &str) -> Self {
    match input {
      "api_error" => Self::API,
      "card_error" => Self::Card,
      "idempotency_error" => Self::Idempotency,
      "invalid_request_error" => Self::InvalidRequest,
      _ => Self::None
    }
  }

  /// Returns the original value.
  pub const fn original_str(&self) -> &'static str {
    match self {
      Self::API => "api_error",
      Self::Card => "card_error",
      Self::Idempotency => "idempotency_error",
      Self::InvalidRequest => "invalid_request_error",
      Self::None => ""
    }
  }
}

#[derive(Serialize)]
pub struct Info {
  pub http_code: HTTPCodes,
  pub r#type: Types,
  pub code: Codes,
  pub message: String
}

#[doc(hidden)]
impl Info {
  pub fn create(status: u16, json_text: &str) -> Option<Self> {
    let json = match serde_json::from_str::<serde_json::Value>(json_text) {
      Ok(r) => {
        if r["error"] == json!(null) {
          r
        } else {
          r["error"].clone()
        }
      },
      Err(_) => return None
    };

    Some(Self {
      http_code: HTTPCodes::from_status(status),
      r#type: Types::from_str(json["type"].as_str().unwrap_or("")),
      code: Codes::from_str(json["code"].as_str().unwrap_or("")),
      message: json["message"].as_str().unwrap_or("").to_string()
    })
  }
}