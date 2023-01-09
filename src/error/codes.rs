//! All available error codes from 01/08/2023
//! 
//! [Official Stripe error code list](https://stripe.com/docs/error-codes)

pub const ACCOUNT_COUNTRY_INVALID_ADDRESS: &str = "account_country_invalid_address";
pub const ACCOUNT_ERROR_COUNTRY_CHANGE_REQUIRES_ADDITIONAL_STEPS: &str = "account_error_country_change_requires_additional_steps";
pub const ACCOUNT_INFORMATION_MISMATCH: &str = "account_information_mismatch";
pub const ACCOUNT_INVALID: &str = "account_invalid";
pub const ACCOUNT_NUMBER_INVALID: &str = "account_number_invalid";

pub const ACSS_DEBIT_SESSION_INCOMPLETE: &str = "acss_debit_session_incomplete";

pub const ALIPAY_UPGRADE_REQUIRED: &str = "alipay_upgrade_required";

pub const AMOUNT_TOO_LARGE: &str = "amount_too_large";
pub const AMOUNT_TOO_SMALL: &str = "amount_too_small";

pub const API_KEY_EXPIRED: &str = "api_key_expired";

pub const AUTHENTICATION_REQUIRED: &str = "authentication_required";

pub const BALANCE_INSUFFICIENT: &str = "balance_insufficient";

pub const BANK_ACCOUNT_BAD_ROUTING_NUMBERS: &str = "bank_account_bad_routing_numbers";
pub const BANK_ACCOUNT_DECLINED: &str = "bank_account_declined";
pub const BANK_ACCOUNT_EXISTS: &str = "bank_account_exists";
pub const BANK_ACCOUNT_RESTRICTED: &str = "bank_account_restricted";
pub const BANK_ACCOUNT_UNUSABLE: &str = "bank_account_unusable";
pub const BANK_ACCOUNTUNVERIFIED: &str = "bank_account_unverified";
pub const BANK_ACCOUNT_VERIFICATION_FAILED: &str = "bank_account_verification_failed";

pub const BILLING_INVALID_MANDATE: &str = "billing_invalid_mandate";

pub const BITCOIN_UPGRADE_REQUIRED: &str = "bitcoin_upgrade_required";

pub const CARD_DECLINE_RATE_LIMIT_EXCEEDED: &str = "card_decline_rate_limit_exceeded";
pub const CARD_DECLINED: &str = "card_declined";

pub const CARDHOLDER_PHONE_NUMBER_REQUIRED: &str = "cardholder_phone_number_required";

pub const CHARGE_ALREADY_CAPTURED: &str = "charge_already_captured";
pub const CHARGE_ALREADY_REFUNDED: &str = "charge_already_refunded";
pub const CHARGE_DISPUTED: &str = "charge_disputed";
pub const CHARGE_EXCEEDS_SOURCE_LIMIT: &str = "charge_exceeds_source_limit";
pub const CHARGE_EXPIRED_FOR_CAPTURE: &str = "charge_expired_for_capture";
pub const CHARGE_INVALID_PARAMETER: &str = "charge_invalid_parameter";

pub const CLEARING_CODE_UNSUPPORTED: &str = "clearing_code_unsupported";

pub const COUNTRY_CODE_INVALID: &str = "country_code_invalid";
pub const COUNTRY_UNSUPPORTED: &str = "country_unsupported";

pub const COUPON_EXPIRED: &str = "coupon_expired";

pub const CUSTOMER_MAX_PAYMENT_METHODS: &str = "customer_max_payment_methods";
pub const CUSTOMER_MAX_SUBSCRIPTIONS: &str = "customer_max_subscriptions";

pub const DEBIT_NOT_AUTHORIZED: &str = "debit_not_authorized";

pub const EMAIL_INVALID: &str = "email_invalid";

pub const EXPIRED_CARD: &str = "expired_card";

pub const IDEMPOTENCY_KEY_IN_USE: &str = "idempotency_key_in_use";

pub const INCORRECT_ADDRESS: &str = "incorrect_address";
pub const INCORRECT_CVC: &str = "incorrect_cvc";
pub const INCORRECT_NUMBER: &str = "incorrect_number";
pub const INCORRECT_ZIP: &str = "incorrect_zip";

pub const INSTANT_PAYOUTS_CONFIG_DISABLED: &str = "instant_payouts_config_disabled";
pub const INSTANT_PAYOUTS_CURRENCY_DISABLED: &str = "instant_payouts_currency_disabled";
pub const INSTANT_PAYOUTS_LIMIT_EXCEEDED: &str = "instant_payouts_limit_exceeded";
pub const INSTANT_PAYOUTS_UNSUPPORTED: &str = "instant_payouts_unsupported";

pub const INSUFFICIENT_FUNDS: &str = "insufficient_funds";

pub const INTENT_INVALID_STATE: &str = "intent_invalid_state";
pub const INTENT_VERIFICATION_METHOD_MISSING: &str = "intent_verification_method_missing";

pub const INVALID_CARD_TYPE: &str = "invalid_card_type";
pub const INVALID_CHARACTERS: &str = "invalid_characters";
pub const INVALID_CHARGE_AMOUNT: &str = "invalid_charge_amount";
pub const INVALID_CVC: &str = "invalid_cvc";
pub const INVALID_EXPIRY_MONTH: &str = "invalid_expiry_month";
pub const INVALID_EXPIRY_YEAR: &str = "invalid_expiry_year";
pub const INVALID_NUMBER: &str = "invalid_number";
pub const INVALID_SOURCE_USAGE: &str = "invalid_source_usage";

pub const INVOICE_NO_CUSTOMER_LINE_ITEMS: &str = "invoice_no_customer_line_items";
pub const INVOICE_NO_PAYMENT_METHOD_TYPES: &str = "invoice_no_payment_method_types";
pub const INVOICE_NO_SUBSCRIPTION_LINE_ITEMS: &str = "invoice_no_subscription_line_items";
pub const INVOICE_NOT_EDITABLE: &str = "invoice_not_editable";
pub const INVOICE_ON_BEHALF_OF_NOT_EDITABLE: &str = "invoice_on_behalf_of_not_editable";
pub const INVOICE_PAYMENT_INTENT_REQUIRES_ACTION: &str = "invoice_payment_intent_requires_action";
pub const INVOICE_UPCOMING_NONE: &str = "invoice_upcoming_none";

pub const LIVEMODE_MISMATCH: &str = "livemode_mismatch";

pub const LOCK_TIMEOUT: &str = "lock_timeout";

pub const MISSING: &str = "missing";

pub const NO_ACCOUNT: &str = "no_account";

pub const NOT_ALLOWED_ON_STANDARD_ACCOUNT: &str = "not_allowed_on_standard_account";

pub const OUT_OF_INVENTORY: &str = "out_of_inventory";

pub const OWNERSHIP_DECLARATION_NOT_ALLOWED: &str = "ownership_declaration_not_allowed";

pub const PARAMETER_INVALID_EMPTY: &str = "parameter_invalid_empty";
pub const PARAMETER_INVALID_INTEGER: &str = "parameter_invalid_integer";
pub const PARAMETER_INVALID_STRING_BLANK: &str = "parameter_invalid_string_blank";
pub const PARAMETER_INVALID_STRING_EMPTY: &str = "parameter_invalid_string_empty";
pub const PARAMETER_MISSING: &str = "parameter_missing";
pub const PARAMETER_UNKNOWN: &str = "paremeter_unknown";

pub const PARAMETERS_EXCLUSIVE: &str = "parameters_exclusive";

pub const PAYMENT_INTENT_ACTION_REQUIRED: &str = "payment_intent_action_required";
pub const PAYMENT_INTENT_AUTHENTICATION_REQUIRED: &str = "payment_intent_authentication_required";
pub const PAYMENT_INTENT_INCOMPATIBLE_PAYMENT_METHOD: &str = "payment_intent_incompatible_payment_method";
pub const PAYMENT_INTENT_INVALID_PARAMETER: &str = "payment_intent_invalid_parameter";
pub const PAYMENT_INTENT_KONBINI_REJECTED_CONFIRMATION_NUMBER: &str = "payment_intent_konbini_rejected_confirmation_number";
pub const PAYMENT_INTENT_MANDATE_INVALID: &str = "payment_intent_mandate_invalid";
pub const PAYMENT_INTENT_PAYMENT_ATTEMPT_EXPIRED: &str = "payment_intent_payment_attempt_expired";
pub const PAYMENT_INTENT_PAYMENT_ATTEMPT_FAILED: &str = "payment_intent_payment_attempt_failed";
pub const PAYMENT_INTENT_UNEXPECTED_STATE: &str = "payment_intent_unexpected_state";

pub const PAYMENT_METHOD_BANK_ACCOUNT_ALREADY_VERIFIED: &str = "payment_method_bank_account_already_verified";
pub const PAYMENT_METHOD_BANK_ACCOUNT_BLOCKED: &str = "payment_method_bank_account_blocked";
pub const PAYMENT_METHOD_BILLING_DETAILS_ADDRESS_MISSING: &str = "payment_method_billing_details_address_missing";
pub const PAYMENT_METHOD_CURRENCY_MISMATCH: &str = "payment_method_currency_mismatch";
pub const PAYMENT_METHOD_CUSTOMER_DECLINE: &str = "payment_method_customer_decline";
pub const PAYMENT_METHOD_INVALID_PARAMETER: &str = "payment_method_invalid_parameter";
pub const PAYMENT_METHOD_INVALID_PARAMETER_TESTMODE: &str = "payment_method_invalid_parameter_testmode";
pub const PAYMENT_METHOD_MICRODEPOSIT_FAILED: &str = "payment_method_microdeposit_failed";
pub const PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_AMOUNTS_INVALID: &str = "payment_method_microdeposit_verification_amounts_invalid";
pub const PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_AMOUNTS_MISMATCH: &str = "payment_method_microdeposit_verification_amounts_mismatch";
pub const PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_ATTEMPTS_EXCEEDED: &str = "payment_method_microdeposit_verification_attempts_exceeded";
pub const PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_DESCRIPTOR_CODE_MISMATCH: &str = "payment_method_microdeposit_verification_descriptor_code_mismatch";
pub const PAYMENT_METHOD_MICRODEPOSIT_VERIFICATION_TIMEOUT: &str = "payment_method_microdeposit_verification_timeout";
pub const PAYMENT_METHOD_PROVIDER_DECLINE: &str = "payment_method_provider_decline";
pub const PAYMENT_METHOD_PROVIDER_TIMEOUT: &str = "payment_method_provider_timeout";
pub const PAYMENT_METHOD_UNACTIVATED: &str = "payment_method_unactivated";
pub const PAYMENT_METHOD_UNEXPECTED_STATE: &str = "payment_method_unexpected_state";
pub const PAYMENT_METHOD_UNSUPPORTED_TYPE: &str = "payment_method_unsupported_type";

pub const PAYOUTS_NOT_ALLOWED: &str = "payouts_not_allowed";

pub const PLATFORM_ACCOUNT_REQUIRED: &str = "platform_account_required";
pub const PLATFORM_API_KEY_EXPIRED: &str = "platform_api_key_expired";

pub const POSTAL_CODE_INVALID: &str = "postal_code_invalid";

pub const PROCESSING_ERROR: &str = "processing_error";

pub const PRODUCT_INACTIVE: &str = "product_inactive";

pub const RATE_LIMIT: &str = "rate_limit";

pub const REFER_TO_CUSTOMER: &str = "refer_to_customer";

pub const REFUND_DISPUTED_PAYMENT: &str = "refund_disputed_payment";

pub const RESOURCE_ALREADY_EXISTS: &str = "resource_already_exists";
pub const RESOURCE_MISSING: &str = "resource_missing";

pub const RETURN_INTENT_ALREADY_PROCESSED: &str = "return_intent_already_processed";

pub const ROUTING_NUMBER_INVALID: &str = "routing_number_invalid";

pub const SECRET_KEY_REQUIRED: &str = "secret_key_required";

pub const SEPA_UNSUPPORTED_ACCOUNT: &str = "sepa_unsupported_account";

pub const SETUP_ATTEMPT_FAILED: &str = "setup_attempt_failed";

pub const SETUP_INTENT_AUTHENTICATION_FAILURE: &str = "setup_intent_authentication_failure";
pub const SETUP_INTENT_INVALID_PARAMETER: &str = "setup_intent_invalid_parameter";
pub const SETUP_INTENT_SETUP_ATTEMPT_EXPIRED: &str = "setup_intent_setup_attempt_expired";
pub const SETUP_INTENT_UNEXPECTED_STATE: &str = "setup_intent_unexpected_state";

pub const SHIPPING_CALCULATION_FAILED: &str = "shipping_calculation_failed";

pub const SKU_INACTIVE: &str = "sku_inactive";

pub const STATE_UNSUPPORTED: &str = "state_unsupported";

pub const STATUS_TRANSITION_INVALID: &str = "status_transition_invalid";

pub const TAX_ID_INVALID: &str = "tax_id_invalid";

pub const TAXES_CALCULATION_FAILED: &str = "taxes_calculation_failed";

pub const TERMINAL_LOCATION_COUNTRY_UNSUPPORTED: &str = "terminal_location_country_unsupported";

pub const TESTMODE_CHARGES_ONLY: &str = "testmode_charges_only";

pub const TLS_VERSION_UNSUPPORTED: &str = "tls_version_unsupported";

pub const TOKEN_ALREADY_USED: &str = "token_already_used";
pub const TOKEN_IN_USE: &str = "token_in_use";

pub const TRANSFER_SOURCE_BALANCE_PARAMETERS_MISMATCH: &str = "transfer_source_balance_parameters_mismatch";

pub const TRANSFERS_NOT_ALLOWED: &str = "transfers_not_allowed";

pub const URL_INVALID: &str = "url_invalid";