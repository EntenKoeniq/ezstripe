#[macro_use]
extern crate serde_json;

pub mod error;
pub mod payment_intent;

/// The secret key for authentication is stored in this static variable.
#[doc(hidden)]
pub static mut SECRET_KEY: String = String::new();

/// Returns the secret key for authentication.
#[doc(hidden)]
pub fn get_secret() -> &'static String {
  unsafe {
    &SECRET_KEY
  }
}

/// Set a new secret to authenticate with Stripe's API.
pub unsafe fn set_secret(key: &str) {
  SECRET_KEY = String::from(key);
}

/// Create an easy body format for API requests.
/// 
/// # Example
/// ```
/// #[macro_use] extern crate ezstripe;
/// 
/// fn main() {
///   let body = ezbody!(
///       "amount" => 2000,
///       "currency" => "eur"
///     );
/// 
///   println!("{}", body); // amount=2000;currency=eur;
/// }
/// ```
#[macro_export]
macro_rules! ezbody {
  {$($k: expr => $v: expr),* $(,)?} => {
    {
      let mut result = String::new();

      $(
        result += format!("{}={};", $k, $v).as_str();
      )*

      result
    }
  };
}