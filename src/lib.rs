#[macro_use]
extern crate serde_json;

use std::sync::Mutex;

pub mod error;
pub mod payment_intent;

#[doc(hidden)]
/// The secret key for authentication is stored in this static variable.
pub static mut SECRET_KEY: Mutex<String> = Mutex::new(String::new());

#[doc(hidden)]
/// Returns the secret key for authentication.
pub fn get_secret() -> &'static String {
  unsafe {
    SECRET_KEY.get_mut().unwrap()
  }
}

/// Set a new secret to authenticate with Stripe's API.
pub unsafe fn set_secret(key: &str) -> Result<(), ()> {
  if let Ok(mut r) = SECRET_KEY.lock() {
    *r = String::from(key);  
  } else {
    return Err(());
  }
  
  Ok(())
}

/// Create an easy body format for API requests.
/// 
/// # Example
/// ```
/// #[macro_use]
/// extern crate ezstripe;
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