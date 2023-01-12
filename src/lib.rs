#[doc(hidden)]
pub mod helper;
pub mod balance;
pub mod mandate;
pub mod payment_intent;
pub mod payout;
pub mod refund;
pub mod error;

#[doc(hidden)]
pub static mut DEBUG: bool = false;
#[doc(hidden)]
pub fn get_debug() -> bool {
  unsafe {
    DEBUG
  }
}

/// **(STRONGLY RECOMMENDED IN DEVELOPMENT)**
/// 
/// Writes errors (if any) to your console.
/// 
/// # Arguments
/// 
/// * `val` - "true" if you want to enable it, "false" otherwise. (default: false)
/// 
/// # Example
/// ```
/// fn main() {
///   unsafe {
///     ezstripe::set_debug(true);
///   };
/// 
///   // ...
/// }
/// ```
pub unsafe fn set_debug(val: bool) {
  DEBUG = val;
}

include!("client.rs");

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