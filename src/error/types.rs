//! All available error types from 01/08/2023
//! 
//! [Official Stripe error types list](https://stripe.com/docs/api/errors)

pub const API_ERROR: &str = "api_error";
pub const CARD_ERROR: &str = "card_error";
pub const IDEMPOTENCY_ERROR: &str = "idempotency_error";
pub const INVALID_REQUEST_ERROR: &str = "invalid_request_error";