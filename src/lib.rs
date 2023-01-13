#[doc(hidden)]
pub mod helper;
#[cfg(feature = "balance")]
pub mod balance;
#[cfg(feature = "mandate")]
pub mod mandate;
#[cfg(feature = "payment_intent")]
pub mod payment_intent;
#[cfg(feature = "payout")]
pub mod payout;
#[cfg(feature = "refund")]
pub mod refund;
pub mod error;

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