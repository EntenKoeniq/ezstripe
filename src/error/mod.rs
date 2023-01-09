use serde::Serialize;

mod types;
mod codes;

/// A list of possible HTTP errors.
#[derive(Serialize)]
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
  /// * `input` - The HTTP status value from Stripe's response
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

/// All available error codes from 01/08/2023
/// 
/// [Official Stripe error code list](https://stripe.com/docs/error-codes)
#[derive(Serialize)]
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
  /// * `input` - The "code" value from Stripe's response
  pub fn from_str(input: &str) -> Self {
    match input {
      codes::ACCOUNT_COUNTRY_INVALID_ADDRESS => Self::AccountCountryInvalidAddress,
      codes::ACCOUNT_ERROR_COUNTRY_CHANGE_REQUIRES_ADDITIONAL_STEPS => Self::AccountErrorCountryChangeRequiresAdditionalSteps,
      codes::ACCOUNT_INFORMATION_MISMATCH => Self::AccountInformationMismatch,
      codes::ACCOUNT_INVALID => Self::AccountInvalid,
      codes::ACCOUNT_NUMBER_INVALID => Self::AccountNumberInvalid,
      codes::ACSS_DEBIT_SESSION_INCOMPLETE => Self::AcssDebitSessionIncomplete,
      codes::ALIPAY_UPGRADE_REQUIRED => Self::AlipayUpgradeRequired,
      codes::AMOUNT_TOO_LARGE => Self::AmountTooLarge,
      codes::AMOUNT_TOO_SMALL => Self::AmountTooSmall,
      codes::API_KEY_EXPIRED => Self::ApiKeyExpired,
      codes::AUTHENTICATION_REQUIRED => Self::AuthenticationRequired,
      codes::BALANCE_INSUFFICIENT => Self::BalanceInsufficient,
      codes::BANK_ACCOUNT_BAD_ROUTING_NUMBERS => Self::BankAccountBadRoutingNumbers,
      codes::BANK_ACCOUNT_DECLINED => Self::BankAccountDeclined,
      codes::BANK_ACCOUNT_EXISTS => Self::BankAccountExists,
      codes::BANK_ACCOUNT_RESTRICTED => Self::BankAccountRestricted,
      codes::BANK_ACCOUNT_UNUSABLE => Self::BankAccountUnusable,
      codes::BANK_ACCOUNTUNVERIFIED => Self::BankAccountUnverified,
      codes::BANK_ACCOUNT_VERIFICATION_FAILED => Self::BankAccountVerificationFailed,
      codes::BILLING_INVALID_MANDATE => Self::BillingInvalidMandate,
      codes::BITCOIN_UPGRADE_REQUIRED => Self::BitcoinUpgradeRequired,
      codes::CARD_DECLINE_RATE_LIMIT_EXCEEDED => Self::CardDeclineRateLimitExceeded,
      codes::CARD_DECLINED => Self::CardDeclined,
      codes::CARDHOLDER_PHONE_NUMBER_REQUIRED => Self::CardholderPhoneNumberRequired,
      codes::CHARGE_ALREADY_CAPTURED => Self::ChargeAlreadyCaptured,
      codes::CHARGE_ALREADY_REFUNDED => Self::ChargeAlreadyRefunded,
      codes::CHARGE_DISPUTED => Self::ChargeDisputed,
      codes::CHARGE_EXCEEDS_SOURCE_LIMIT => Self::ChargeExceedsSourceLimit,
      codes::CHARGE_EXPIRED_FOR_CAPTURE => Self::ChargeExpiredForCapture,
      codes::CHARGE_INVALID_PARAMETER => Self::ChargeInvalidParamter,
      codes::CLEARING_CODE_UNSUPPORTED => Self::ClearingCodeUnsupported,
      codes::COUNTRY_CODE_INVALID => Self::CountryCodeInvalid,
      codes::COUNTRY_UNSUPPORTED => Self::CountryUnsupported,
      codes::COUPON_EXPIRED => Self::CouponExpired,
      codes::CUSTOMER_MAX_PAYMENT_METHODS => Self::CustomerMaxPaymentMethods,
      codes::CUSTOMER_MAX_SUBSCRIPTIONS => Self::CustomerMaxSubscriptions,
      codes::DEBIT_NOT_AUTHORIZED => Self::DebitNotAuthorized,
      codes::EMAIL_INVALID => Self::EmailInvalid,
      codes::EXPIRED_CARD => Self::ExpiredCard,
      codes::IDEMPOTENCY_KEY_IN_USE => Self::IdempotencyKeyInUse,
      codes::INCORRECT_ADDRESS => Self::IncorrectAddress,
      codes::INCORRECT_CVC => Self::IncorrectCVC,
      codes::INCORRECT_NUMBER => Self::IncorrectNumber,
      codes::INCORRECT_ZIP => Self::IncorrectZip,
      codes::INSTANT_PAYOUTS_CONFIG_DISABLED => Self::InstantPayoutsConfigDisabled,
      codes::INSTANT_PAYOUTS_CURRENCY_DISABLED => Self::InstantPayoutsCurrencyDisabled,
      codes::INSTANT_PAYOUTS_LIMIT_EXCEEDED => Self::InstantPayoutsLimitExceeded,
      codes::INSTANT_PAYOUTS_UNSUPPORTED => Self::InstantPayoutsUnsupported,
      codes::INSUFFICIENT_FUNDS => Self::InsufficientFunds,
      codes::INTENT_INVALID_STATE => Self::IntentInvalidState,
      codes::INTENT_VERIFICATION_METHOD_MISSING => Self::IntentVerificationMethodMissing,
      codes::INVALID_CARD_TYPE => Self::InvalidCardType,
      codes::INVALID_CHARACTERS => Self::InvalidCharacters,
      codes::INVALID_CHARGE_AMOUNT => Self::InvalidChargeAmount,
      codes::INVALID_CVC => Self::InvalidCVC,
      codes::INVALID_EXPIRY_MONTH => Self::InvalidExpiryMonth,
      codes::INVALID_EXPIRY_YEAR => Self::InvalidExpiryYear,
      codes::INVALID_NUMBER => Self::InvalidNumber,
      codes::INVALID_SOURCE_USAGE => Self::InvalidSourceUsage,
      codes::INVOICE_NO_CUSTOMER_LINE_ITEMS => Self::InvoiceNoCustomerLineItems,
      codes::INVOICE_NO_PAYMENT_METHOD_TYPES => Self::InvoiceNoPaymentMethodTypes,
      codes::INVOICE_NO_SUBSCRIPTION_LINE_ITEMS => Self::InvoiceNoSubscriptionLineItems,
      codes::INVOICE_NOT_EDITABLE => Self::InvoiceNotEditable,
      codes::INVOICE_ON_BEHALF_OF_NOT_EDITABLE => Self::InvoiceOnBehalfOfNotEditable,
      codes::INVOICE_PAYMENT_INTENT_REQUIRES_ACTION => Self::InvoicePaymentIntentRequiresAction,
      codes::INVOICE_UPCOMING_NONE => Self::InvoiceUpcomingNone,
      codes::LIVEMODE_MISMATCH => Self::LivemodeMismatch,
      codes::LOCK_TIMEOUT => Self::LockTimeout,
      codes::MISSING => Self::Missing,
      codes::NO_ACCOUNT => Self::NoAccount,
      codes::NOT_ALLOWED_ON_STANDARD_ACCOUNT => Self::NotAllowedOnStandardAccount,
      codes::OUT_OF_INVENTORY => Self::OutOfInventory,
      codes::OWNERSHIP_DECLARATION_NOT_ALLOWED => Self::OwnershipDeclarationNotAllowed,
      codes::PARAMETER_INVALID_EMPTY => Self::ParameterInvalidEmpty,
      codes::PARAMETER_INVALID_INTEGER => Self::ParameterInvalidInteger,
      codes::PARAMETER_INVALID_STRING_BLANK => Self::ParameterInvalidStringBlank,
      codes::PARAMETER_INVALID_STRING_EMPTY => Self::ParameterInvalidStringEmpty,
      codes::PARAMETER_MISSING => Self::ParameterMissing,
      codes::PARAMETER_UNKNOWN => Self::ParameterUnknown,
      codes::PARAMETERS_EXCLUSIVE => Self::ParametersExclusive,
      codes::PAYMENT_INTENT_ACTION_REQUIRED => Self::PaymentIntentActionRequired,
      codes::PAYMENT_INTENT_AUTHENTICATION_REQUIRED => Self::PaymentIntentAuthenticationFailure,
      codes::PAYMENT_INTENT_INCOMPATIBLE_PAYMENT_METHOD => Self::PaymentIntentIncompatiblePaymentMethod,
      codes::PAYMENT_INTENT_INVALID_PARAMETER => Self::PaymentIntentInvalidParameter,
      codes::PAYMENT_INTENT_KONBINI_REJECTED_CONFIRMATION_NUMBER => Self::PaymentIntentKonbiniRejectedConfirmationNumber,
      codes::PAYMENT_INTENT_MANDATE_INVALID => Self::PaymentIntentMandateInvalid,
      codes::PAYMENT_INTENT_PAYMENT_ATTEMPT_EXPIRED => Self::PaymentIntentPaymentAttemptExpired,
      codes::PAYMENT_INTENT_PAYMENT_ATTEMPT_FAILED => Self::PaymentIntentPaymentAttemptFailed,
      codes::PAYMENT_INTENT_UNEXPECTED_STATE => Self::PaymentIntentUnexpectedState,
      codes::PAYMENT_METHOD_BANK_ACCOUNT_ALREADY_VERIFIED => Self::PaymentMethodBankAccountAlreadyVerified,
      codes::PAYMENT_METHOD_BANK_ACCOUNT_BLOCKED => Self::PaymentMethodBankAccountBlocked,
      codes::PAYMENT_METHOD_BILLING_DETAILS_ADDRESS_MISSING => Self::PaymentMethodBillingDetailsAddressMissing,
      codes::PAYMENT_METHOD_CURRENCY_MISMATCH => Self::PaymentMethodCurrencyMismatch,
      codes::PAYMENT_METHOD_CUSTOMER_DECLINE => Self::PaymentMethodCustomerDecline,
      codes::PAYMENT_METHOD_INVALID_PARAMETER => Self::PaymentMethodInvalidParameter,
      codes::PAYMENT_METHOD_INVALID_PARAMETER_TESTMODE => Self::PaymentMethodInvalidParameterTestmode,
      codes::PAYMENT_METHOD_MICRODEPOSIT_FAILED => Self::PaymentMethodMicrodepositFailed,
      codes::PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_AMOUNTS_INVALID => Self::PaymentMethodMicrodepositVerificationAmountsInvalid,
      codes::PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_AMOUNTS_MISMATCH => Self::PaymentMethodMicrodepositVerificationAmountsMismatch,
      codes::PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_ATTEMPTS_EXCEEDED => Self::PaymentMethodMicrodepositVerificationAttemptsExceeded,
      codes::PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_DESCRIPTOR_CODE_MISMATCH => Self::PaymentMethodMicrodepositVerificationDescriptorCodeMismatch,
      codes::PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_TIMEOUT => Self::PaymentMethodMicrodepositVerificationTimeout,
      codes::PAYMENT_METHOD_PROVIDER_DECLINE => Self::PaymentMethodProviderDecline,
      codes::PAYMENT_METHOD_PROVIDER_TIMEOUT => Self::PaymentMethodProviderTimeout,
      codes::PAYMENT_METHOD_UNACTIVATED => Self::PaymentMethodUnactivated,
      codes::PAYMENT_METHOD_UNEXPECTED_STATE => Self::PaymentMethodUnexpectedState,
      codes::PAYMENT_METHOD_UNSUPPORTED_TYPE => Self::PaymentMethodUnsupportedType,
      codes::PAYOUTS_NOT_ALLOWED => Self::PayoutsNotAllowed,
      codes::PLATFORM_ACCOUNT_REQUIRED => Self::PlatformAccountRequired,
      codes::PLATFORM_API_KEY_EXPIRED => Self::PlatformApiKeyExpired,
      codes::POSTAL_CODE_INVALID => Self::PostalCodeInvalid,
      codes::PROCESSING_ERROR => Self::ProcessingError,
      codes::PRODUCT_INACTIVE => Self::ProductInactive,
      codes::RATE_LIMIT => Self::RateLimit,
      codes::REFER_TO_CUSTOMER => Self::ReferToCustomer,
      codes::REFUND_DISPUTED_PAYMENT => Self::RefundDisputedPayment,
      codes::RESOURCE_ALREADY_EXISTS => Self::ResourceAlreadyExists,
      codes::RESOURCE_MISSING => Self::ResourceMissing,
      codes::RETURN_INTENT_ALREADY_PROCESSED => Self::ReturnIntentAlreadyProcessed,
      codes::ROUTING_NUMBER_INVALID => Self::RoutingNumberInvalid,
      codes::SECRET_KEY_REQUIRED => Self::SecretKeyRequired,
      codes::SEPA_UNSUPPORTED_ACCOUNT => Self::SepaUnsupportedAccount,
      codes::SETUP_ATTEMPT_FAILED => Self::SetupAttemptFailed,
      codes::SETUP_INTENT_AUTHENTICATION_FAILURE => Self::SetupIntentAuthenticationFailure,
      codes::SETUP_INTENT_INVALID_PARAMETER => Self::SetupIntentInvalidParameter,
      codes::SETUP_INTENT_SETUP_ATTEMPT_EXPIRED => Self::SetupIntentSetupAttemptExpired,
      codes::SETUP_INTENT_UNEXPECTED_STATE => Self::SetupIntentUnexpectedState,
      codes::SHIPPING_CALCULATION_FAILED => Self::ShippingCalculationFailed,
      codes::SKU_INACTIVE => Self::SkuInactive,
      codes::STATE_UNSUPPORTED => Self::StateUnsupported,
      codes::STATUS_TRANSITION_INVALID => Self::StatusTransitionInvalid,
      codes::TAX_ID_INVALID => Self::TaxIdInvalid,
      codes::TAXES_CALCULATION_FAILED => Self::TaxesCalculationFailed,
      codes::TERMINAL_LOCATION_COUNTRY_UNSUPPORTED => Self::TerminalLocationCountryUnsupported,
      codes::TESTMODE_CHARGES_ONLY => Self::TestmodeChargesOnly,
      codes::TLS_VERSION_UNSUPPORTED => Self::TlsVersionUnsupported,
      codes::TOKEN_ALREADY_USED => Self::TokenAlreadyUsed,
      codes::TOKEN_IN_USE => Self::TokenInUse,
      codes::TRANSFER_SOURCE_BALANCE_PARAMETERS_MISMATCH => Self::TransferSourceBalanceParametersMismatch,
      codes::TRANSFERS_NOT_ALLOWED => Self::TransfersNotAllowed,
      codes::URL_INVALID => Self::UrlInvalid,
      _ => Self::None
    }
  }

  /// Returns the original value.
  pub const fn original_str(&self) -> &'static str {    
    match self {
      Self::AccountCountryInvalidAddress => codes::ACCOUNT_COUNTRY_INVALID_ADDRESS,
      Self::AccountErrorCountryChangeRequiresAdditionalSteps => codes::ACCOUNT_ERROR_COUNTRY_CHANGE_REQUIRES_ADDITIONAL_STEPS,
      Self::AccountInformationMismatch => codes::ACCOUNT_INFORMATION_MISMATCH,
      Self::AccountInvalid => codes::ACCOUNT_INVALID,
      Self::AccountNumberInvalid => codes::ACCOUNT_NUMBER_INVALID,
      Self::AcssDebitSessionIncomplete => codes::ACSS_DEBIT_SESSION_INCOMPLETE,
      Self::AlipayUpgradeRequired => codes::ALIPAY_UPGRADE_REQUIRED,
      Self::AmountTooLarge => codes::AMOUNT_TOO_LARGE,
      Self::AmountTooSmall => codes::AMOUNT_TOO_SMALL,
      Self::ApiKeyExpired => codes::API_KEY_EXPIRED,
      Self::AuthenticationRequired => codes::AUTHENTICATION_REQUIRED,
      Self::BalanceInsufficient => codes::BALANCE_INSUFFICIENT,
      Self::BankAccountBadRoutingNumbers => codes::BANK_ACCOUNT_BAD_ROUTING_NUMBERS,
      Self::BankAccountDeclined => codes::BANK_ACCOUNT_DECLINED,
      Self::BankAccountExists => codes::BANK_ACCOUNT_EXISTS,
      Self::BankAccountRestricted => codes::BANK_ACCOUNT_RESTRICTED,
      Self::BankAccountUnusable => codes::BANK_ACCOUNT_UNUSABLE,
      Self::BankAccountUnverified => codes::BANK_ACCOUNTUNVERIFIED,
      Self::BankAccountVerificationFailed => codes::BANK_ACCOUNT_VERIFICATION_FAILED,
      Self::BillingInvalidMandate => codes::BILLING_INVALID_MANDATE,
      Self::BitcoinUpgradeRequired => codes::BITCOIN_UPGRADE_REQUIRED,
      Self::CardDeclineRateLimitExceeded => codes::CARD_DECLINE_RATE_LIMIT_EXCEEDED,
      Self::CardDeclined => codes::CARD_DECLINED,
      Self::CardholderPhoneNumberRequired => codes::CARDHOLDER_PHONE_NUMBER_REQUIRED,
      Self::ChargeAlreadyCaptured => codes::CHARGE_ALREADY_CAPTURED,
      Self::ChargeAlreadyRefunded => codes::CHARGE_ALREADY_REFUNDED,
      Self::ChargeDisputed => codes::CHARGE_DISPUTED,
      Self::ChargeExceedsSourceLimit => codes::CHARGE_EXCEEDS_SOURCE_LIMIT,
      Self::ChargeExpiredForCapture => codes::CHARGE_EXPIRED_FOR_CAPTURE,
      Self::ChargeInvalidParamter => codes::CHARGE_INVALID_PARAMETER,
      Self::ClearingCodeUnsupported => codes::CLEARING_CODE_UNSUPPORTED,
      Self::CountryCodeInvalid => codes::COUNTRY_CODE_INVALID,
      Self::CountryUnsupported => codes::COUNTRY_UNSUPPORTED,
      Self::CouponExpired => codes::COUPON_EXPIRED,
      Self::CustomerMaxPaymentMethods => codes::CUSTOMER_MAX_PAYMENT_METHODS,
      Self::CustomerMaxSubscriptions => codes::CUSTOMER_MAX_SUBSCRIPTIONS,
      Self::DebitNotAuthorized => codes::DEBIT_NOT_AUTHORIZED,
      Self::EmailInvalid => codes::EMAIL_INVALID,
      Self::ExpiredCard => codes::EXPIRED_CARD,
      Self::IdempotencyKeyInUse => codes::IDEMPOTENCY_KEY_IN_USE,
      Self::IncorrectAddress => codes::INCORRECT_ADDRESS,
      Self::IncorrectCVC => codes::INCORRECT_CVC,
      Self::IncorrectNumber => codes::INCORRECT_NUMBER,
      Self::IncorrectZip => codes::INCORRECT_ZIP,
      Self::InstantPayoutsConfigDisabled => codes::INSTANT_PAYOUTS_CONFIG_DISABLED,
      Self::InstantPayoutsCurrencyDisabled => codes::INSTANT_PAYOUTS_CURRENCY_DISABLED,
      Self::InstantPayoutsLimitExceeded => codes::INSTANT_PAYOUTS_LIMIT_EXCEEDED,
      Self::InstantPayoutsUnsupported => codes::INSTANT_PAYOUTS_UNSUPPORTED,
      Self::InsufficientFunds => codes::INSUFFICIENT_FUNDS,
      Self::IntentInvalidState => codes::INTENT_INVALID_STATE,
      Self::IntentVerificationMethodMissing => codes::INTENT_VERIFICATION_METHOD_MISSING,
      Self::InvalidCardType => codes::INVALID_CARD_TYPE,
      Self::InvalidCharacters => codes::INVALID_CHARACTERS,
      Self::InvalidChargeAmount => codes::INVALID_CHARGE_AMOUNT,
      Self::InvalidCVC => codes::INVALID_CVC,
      Self::InvalidExpiryMonth => codes::INVALID_EXPIRY_MONTH,
      Self::InvalidExpiryYear => codes::INVALID_EXPIRY_YEAR,
      Self::InvalidNumber => codes::INVALID_NUMBER,
      Self::InvalidSourceUsage => codes::INVALID_SOURCE_USAGE,
      Self::InvoiceNoCustomerLineItems => codes::INVOICE_NO_CUSTOMER_LINE_ITEMS,
      Self::InvoiceNoPaymentMethodTypes => codes::INVOICE_NO_PAYMENT_METHOD_TYPES,
      Self::InvoiceNoSubscriptionLineItems => codes::INVOICE_NO_SUBSCRIPTION_LINE_ITEMS,
      Self::InvoiceNotEditable => codes::INVOICE_NOT_EDITABLE,
      Self::InvoiceOnBehalfOfNotEditable => codes::INVOICE_ON_BEHALF_OF_NOT_EDITABLE,
      Self::InvoicePaymentIntentRequiresAction => codes::INVOICE_PAYMENT_INTENT_REQUIRES_ACTION,
      Self::InvoiceUpcomingNone => codes::INVOICE_UPCOMING_NONE,
      Self::LivemodeMismatch => codes::LIVEMODE_MISMATCH,
      Self::LockTimeout => codes::LOCK_TIMEOUT,
      Self::Missing => codes::MISSING,
      Self::NoAccount => codes::NO_ACCOUNT,
      Self::NotAllowedOnStandardAccount => codes::NOT_ALLOWED_ON_STANDARD_ACCOUNT,
      Self::OutOfInventory => codes::OUT_OF_INVENTORY,
      Self::OwnershipDeclarationNotAllowed => codes::OWNERSHIP_DECLARATION_NOT_ALLOWED,
      Self::ParameterInvalidEmpty => codes::PARAMETER_INVALID_EMPTY,
      Self::ParameterInvalidInteger => codes::PARAMETER_INVALID_INTEGER,
      Self::ParameterInvalidStringBlank => codes::PARAMETER_INVALID_STRING_BLANK,
      Self::ParameterInvalidStringEmpty => codes::PARAMETER_INVALID_STRING_EMPTY,
      Self::ParameterMissing => codes::PARAMETER_MISSING,
      Self::ParameterUnknown => codes::PARAMETER_UNKNOWN,
      Self::ParametersExclusive => codes::PARAMETERS_EXCLUSIVE,
      Self::PaymentIntentActionRequired => codes::PAYMENT_INTENT_ACTION_REQUIRED,
      Self::PaymentIntentAuthenticationFailure => codes::PAYMENT_INTENT_AUTHENTICATION_REQUIRED,
      Self::PaymentIntentIncompatiblePaymentMethod => codes::PAYMENT_INTENT_INCOMPATIBLE_PAYMENT_METHOD,
      Self::PaymentIntentInvalidParameter => codes::PAYMENT_INTENT_INVALID_PARAMETER,
      Self::PaymentIntentKonbiniRejectedConfirmationNumber => codes::PAYMENT_INTENT_KONBINI_REJECTED_CONFIRMATION_NUMBER,
      Self::PaymentIntentMandateInvalid => codes::PAYMENT_INTENT_MANDATE_INVALID,
      Self::PaymentIntentPaymentAttemptExpired => codes::PAYMENT_INTENT_PAYMENT_ATTEMPT_EXPIRED,
      Self::PaymentIntentPaymentAttemptFailed => codes::PAYMENT_INTENT_PAYMENT_ATTEMPT_FAILED,
      Self::PaymentIntentUnexpectedState => codes::PAYMENT_INTENT_UNEXPECTED_STATE,
      Self::PaymentMethodBankAccountAlreadyVerified => codes::PAYMENT_METHOD_BANK_ACCOUNT_ALREADY_VERIFIED,
      Self::PaymentMethodBankAccountBlocked => codes::PAYMENT_METHOD_BANK_ACCOUNT_BLOCKED,
      Self::PaymentMethodBillingDetailsAddressMissing => codes::PAYMENT_METHOD_BILLING_DETAILS_ADDRESS_MISSING,
      Self::PaymentMethodCurrencyMismatch => codes::PAYMENT_METHOD_CURRENCY_MISMATCH,
      Self::PaymentMethodCustomerDecline => codes::PAYMENT_METHOD_CUSTOMER_DECLINE,
      Self::PaymentMethodInvalidParameter => codes::PAYMENT_METHOD_INVALID_PARAMETER,
      Self::PaymentMethodInvalidParameterTestmode => codes::PAYMENT_METHOD_INVALID_PARAMETER_TESTMODE,
      Self::PaymentMethodMicrodepositFailed => codes::PAYMENT_METHOD_MICRODEPOSIT_FAILED,
      Self::PaymentMethodMicrodepositVerificationAmountsInvalid => codes::PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_AMOUNTS_INVALID,
      Self::PaymentMethodMicrodepositVerificationAmountsMismatch => codes::PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_AMOUNTS_MISMATCH,
      Self::PaymentMethodMicrodepositVerificationAttemptsExceeded => codes::PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_ATTEMPTS_EXCEEDED,
      Self::PaymentMethodMicrodepositVerificationDescriptorCodeMismatch => codes::PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_DESCRIPTOR_CODE_MISMATCH,
      Self::PaymentMethodMicrodepositVerificationTimeout => codes::PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_TIMEOUT,
      Self::PaymentMethodProviderDecline => codes::PAYMENT_METHOD_PROVIDER_DECLINE,
      Self::PaymentMethodProviderTimeout => codes::PAYMENT_METHOD_PROVIDER_TIMEOUT,
      Self::PaymentMethodUnactivated => codes::PAYMENT_METHOD_UNACTIVATED,
      Self::PaymentMethodUnexpectedState => codes::PAYMENT_METHOD_UNEXPECTED_STATE,
      Self::PaymentMethodUnsupportedType => codes::PAYMENT_METHOD_UNSUPPORTED_TYPE,
      Self::PayoutsNotAllowed => codes::PAYOUTS_NOT_ALLOWED,
      Self::PlatformAccountRequired => codes::PLATFORM_ACCOUNT_REQUIRED,
      Self::PlatformApiKeyExpired => codes::PLATFORM_API_KEY_EXPIRED,
      Self::PostalCodeInvalid => codes::POSTAL_CODE_INVALID,
      Self::ProcessingError => codes::PROCESSING_ERROR,
      Self::ProductInactive => codes::PRODUCT_INACTIVE,
      Self::RateLimit => codes::RATE_LIMIT,
      Self::ReferToCustomer => codes::REFER_TO_CUSTOMER,
      Self::RefundDisputedPayment => codes::REFUND_DISPUTED_PAYMENT,
      Self::ResourceAlreadyExists => codes::RESOURCE_ALREADY_EXISTS,
      Self::ResourceMissing => codes::RESOURCE_MISSING,
      Self::ReturnIntentAlreadyProcessed => codes::RETURN_INTENT_ALREADY_PROCESSED,
      Self::RoutingNumberInvalid => codes::ROUTING_NUMBER_INVALID,
      Self::SecretKeyRequired => codes::SECRET_KEY_REQUIRED,
      Self::SepaUnsupportedAccount => codes::SEPA_UNSUPPORTED_ACCOUNT,
      Self::SetupAttemptFailed => codes::SETUP_ATTEMPT_FAILED,
      Self::SetupIntentAuthenticationFailure => codes::SETUP_INTENT_AUTHENTICATION_FAILURE,
      Self::SetupIntentInvalidParameter => codes::SETUP_INTENT_INVALID_PARAMETER,
      Self::SetupIntentSetupAttemptExpired => codes::SETUP_INTENT_SETUP_ATTEMPT_EXPIRED,
      Self::SetupIntentUnexpectedState => codes::SETUP_INTENT_UNEXPECTED_STATE,
      Self::ShippingCalculationFailed => codes::SHIPPING_CALCULATION_FAILED,
      Self::SkuInactive => codes::SKU_INACTIVE,
      Self::StateUnsupported => codes::STATE_UNSUPPORTED,
      Self::StatusTransitionInvalid => codes::STATUS_TRANSITION_INVALID,
      Self::TaxIdInvalid => codes::TAX_ID_INVALID,
      Self::TaxesCalculationFailed => codes::TAXES_CALCULATION_FAILED,
      Self::TerminalLocationCountryUnsupported => codes::TERMINAL_LOCATION_COUNTRY_UNSUPPORTED,
      Self::TestmodeChargesOnly => codes::TESTMODE_CHARGES_ONLY,
      Self::TlsVersionUnsupported => codes::TLS_VERSION_UNSUPPORTED,
      Self::TokenAlreadyUsed => codes::TOKEN_ALREADY_USED,
      Self::TokenInUse => codes::TOKEN_IN_USE,
      Self::TransferSourceBalanceParametersMismatch => codes::TRANSFER_SOURCE_BALANCE_PARAMETERS_MISMATCH,
      Self::TransfersNotAllowed => codes::TRANSFERS_NOT_ALLOWED,
      Self::UrlInvalid => codes::URL_INVALID,
      Self::None => ""
    }
  }
}

/// All available error types from 01/08/2023
/// 
/// [Official Stripe error types list](https://stripe.com/docs/api/errors)
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
  /// * `input` - The "type" value from Stripe's response
  pub fn from_str(input: &str) -> Self {
    match input {
      types::API_ERROR => Self::API,
      types::CARD_ERROR => Self::Card,
      types::IDEMPOTENCY_ERROR => Self::Idempotency,
      types::INVALID_REQUEST_ERROR => Self::InvalidRequest,
      _ => Self::None
    }
  }

  /// Returns the original value.
  pub const fn original_str(&self) -> &'static str {
    match self {
      Self::API => types::API_ERROR,
      Self::Card => types::CARD_ERROR,
      Self::Idempotency => types::IDEMPOTENCY_ERROR,
      Self::InvalidRequest => types::INVALID_REQUEST_ERROR,
      Self::None => ""
    }
  }
}

/// All the important information about the error from Stripe.
#[derive(Serialize)]
pub struct Info {
  /// The HTTP status code.
  pub http_code: HTTPCodes,
  /// The "type" value from the error response.
  pub r#type: Types,
  /// The "code" value from the error response.
  pub code: Codes,
  /// The "message" value from the error response.
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