# Changelog

## [0.6.1](https://github.com/EntenKoeniq/ezstripe/compare/0.6.0...0.6.1) (01/18/2023)
- Added
- - Missing error types.

## [0.6.0](https://github.com/EntenKoeniq/ezstripe/compare/0.5.0...0.6.0) (01/17/2023)
- Added
- - Support for `BalanceTransactions`.
- - Feature `balance_transaction`.
- Corrected typos, improved documentation and more...

## [0.5.0](https://github.com/EntenKoeniq/ezstripe/compare/0.4.2...0.5.0) (01/17/2023)
- Updated
- - `ezbody!` (Performance increase up to +40%).
- Added
- - Support for `Disputes`.
- - Feature `dispute`.
- Fixed
- - Documentations (Some examples have `get()` instead of `get_list()`).

## [0.4.2](https://github.com/EntenKoeniq/ezstripe/compare/0.4.1...0.4.2) (01/16/2023)
- All PaymentIntent structures completed.
- Added
- - `#[serde(skip_serializing_if = "Option::is_none")]`.
- - Missing `payment_intent::NextAction` members.
- Fixed
- - Typo and renamed `kombini` to `konbini`.
- Some documentation improvements and more...

## [0.4.1](https://github.com/EntenKoeniq/ezstripe/compare/0.4.0...0.4.1) (01/15/2023)
- Removed
- - Duplicate function with different name.
- Updated
- - `payment_intent::Response::NextAction::r#type` is now `String`.
- - `payment_intent::Response::ProcessingCard::customer_notification` is now `Option<ProcessingCardCustomerNotification>`.
- - `payment_intent::Response::TransferData::amount` is now `u32`.
- Some documentation improvements and more...

## [0.4.0](https://github.com/EntenKoeniq/ezstripe/compare/0.3.2...0.4.0) (01/15/2023)
- Performance improvements
- - 5 threads executed the `retrieve_balance()` and `create_payment_intent()` functions 20 times:
- - - 0.4.0: ~11.5s
- - - 0.3.2: ~15.68s
- Added
- - `Client::new("SECRET_KEY")` to create new clients instead of initializing them.
- - `ResponseList` struct for `list_payment_intent()`, `list_payout()` and `list_refund()`.
- - `get_list()` which return the new `ResponseList` for all `Types::LIST`.
- - `Clone` attribute for `Response`.
- Updated
- - LICENSE
- Some documentation improvements and more...

## [0.3.2](https://github.com/EntenKoeniq/ezstripe/compare/0.3.1...0.3.2) (01/13/2023)
- Updated
- - LICENSE.
- - Authors.
- Changed
- - Repository URL (from ".../xEntenKoeniqx/ezstripe" to "../EntenKoeniq/ezstripe").

## [0.3.1](https://github.com/EntenKoeniq/ezstripe/compare/0.3.0...0.3.1) (01/13/2023)
- Added
- - Optional features.
- Removed
- - Debug and replaced with [log](https://crates.io/crates/log) crate.
- - Unused "json" feature for [reqwest](https://crates.io/crates/reqwest) crate.
- Some documentation improvements, bug fixes and more...

## [0.3.0](https://github.com/EntenKoeniq/ezstripe/compare/0.2.2...0.3.0) (01/12/2023)
- Removed
- - `from_str()` and `original_str()` functions from structs and enums for `Error` (replaced with serde).
- - Constant variables for `Error` (replaced with serde).
- PaymentIntent
- - Changed some values ​​to `Option<>` or from to normal.
- - Added `next_action` to response details.
- Added
- - Balance API.
- - Payout API.
- - Refund API.
- - Mandate API.
- Error
- - Renamed members of `Types`.
- Documentation improvements and other changes...

## [0.2.2](https://github.com/EntenKoeniq/ezstripe/compare/0.2.1...0.2.2) (01/11/2023)
- Added "Debug" attribute for some structs.
- PaymentIntent
- - Added missing Response data `payment_method_options`, `metadata`, `processing` and `setup_future_usage`.
- Error
- - Added missing Response data `param` and `payment_intent`.
- Some bug fixes, documentation improvements and other changes...

## [0.2.1](https://github.com/EntenKoeniq/ezstripe/compare/0.2.0...0.2.1) (01/11/2023)
- Added debug option.
- PaymentIntent:
- - Added `get()` function.
- - Fixed wrong request for `RETRIEVE`.
- - Added `LIST` and `UPDATE` request.
- - Added check if the right method was used.
- - `send()` function returns now `Result<crate::payment_intent::Response, (String, Option<crate::error::Info>)>`
- Other changes...

## [0.2.0](https://github.com/EntenKoeniq/ezstripe/compare/0.1.1...0.2.0) (01/11/2023)
- Complete restructuring.
- PaymentIntent
- - Added more response details.
- - Added more documentation.
- Added more documentation.
- Other changes...

## [0.1.1](https://github.com/EntenKoeniq/ezstripe/compare/0.1.0...0.1.1) (01/09/2023)
- Mutex removed from SECRET_KEY.
- Function go() renamed to send() [payment_intent::___::Info].
- Documentation updated.
- Other changes...

## 0.1.0 (01/09/2023)