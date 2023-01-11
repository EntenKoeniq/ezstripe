use serde::{ Serialize, Deserialize };

/// A list of possible HTTP errors.
#[derive(Deserialize, Serialize, Debug)]
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
  /// 
  /// # Arguments
  /// 
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

include!("codes_enum.rs");

/// All available error types from 01/08/2023
/// 
/// [Official Stripe error types list](https://stripe.com/docs/api/errors)
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Types {
  /// ?
  None,
  /// API errors cover any other type of problem (e.g., a temporary problem with Stripe's servers), and are extremely uncommon.
  ApiError,
  /// Card errors are the most common type of error you should expect to handle. They result when the user enters a card that can't be charged for some reason.
  CardError,
  /// Idempotency errors occur when an Idempotency-Key is re-used on a request that does not match the first request's API endpoint and parameters.
  IdempotencyError,
  /// Invalid request errors arise when your request has invalid parameters.
  InvalidRequestError
}

impl std::fmt::Display for Types {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut result = String::new();

    let self_to_string = format!("{:?}", self);
    for (i, c) in self_to_string.chars().enumerate() {
      if i > 0 && c.is_uppercase() {
        result.push('_');
      }
      result.push(c);
    }

    write!(f, "{}", result.to_lowercase())
  }
}

/// All the important information about the error from Stripe.
#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
  /// The HTTP response status code.
  pub http_code: HTTPCodes,
  /// The type of error returned.
  /// One of `api_error`, `card_error`, `idempotency_error`, or `invalid_request_error`
  pub r#type: Types,
  /// For some errors that could be handled programmatically, a short string indicating the [error code](https://stripe.com/docs/error-codes) reported.
  pub code: Codes,
  /// A human-readable message providing more details about the error.
  /// For card errors, these messages can be shown to your users.
  pub message: String,
  /// If the error is parameter-specific, the parameter related to the error.
  /// For example, you can use this to display a message near the correct form field.
  pub param: String,
  /// The PaymentIntent object for errors returned on a request involving a PaymentIntent.
  pub payment_intent: Option<crate::payment_intent::Response>
}

impl Info {
  #[doc(hidden)]
  pub fn create(status: u16, json_text: &str) -> Option<Self> {
    let json = match serde_json::from_str::<serde_json::Value>(json_text) {
      Ok(r) => {
        if r["error"].is_null() {
          r
        } else {
          r["error"].clone()
        }
      },
      Err(_) => return None
    };

    let payment_intent_json = json["payment_intent"].clone();
    let payment_intent = serde_json::from_value::<Option<crate::payment_intent::Response>>(payment_intent_json)
      .unwrap_or(None);

    let type_json = json["type"].clone();
    let r#type = serde_json::from_value::<Types>(type_json)
      .unwrap_or(Types::None);

    let code_json = json["code"].clone();
    let code = serde_json::from_value::<Codes>(code_json)
      .unwrap_or(Codes::None);

    Some(Self {
      http_code: HTTPCodes::from_status(status),
      r#type,
      code,
      message: json["message"].as_str().unwrap_or("").to_string(),
      param: json["param"].as_str().unwrap_or("").to_string(),
      payment_intent
    })
  }

  /// Get the complete Info as String.
  pub fn to_string(&self) -> Result<String, ()> {
    match serde_json::to_string(self) {
      Ok(r) => Ok(r),
      Err(_) => Err(())
    }
  }
}