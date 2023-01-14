# Changelog

## 0.3.2 (01/13/2023)
- LICENSE and Authors updated.
- Changed Github username (from "xEntenKoeniqx" to "EntenKoeniq").
- Changed repository URL (from ".../xEntenKoeniqx/ezstripe" to "../EntenKoeniq/ezstripe").

## 0.3.1 (01/13/2023)
- Added optional features.
- Removed debug and replaced with [log](https://crates.io/crates/log) crate.
- Removed unused "json" feature for [reqwest](https://crates.io/crates/reqwest) crate.
- Some documentation improvements, bug fixes and more...

## 0.3.0 (01/12/2023)
- Removed
- - `from_str()` and `original_str()` functions from structs and enums for `Error` (replaced with serde).
- - Constant variables for `Error` (replaced with serde).
- PaymentIntent
- - Changed some values ​​to `Option<>` or from to normal.
- - Added `next_action` to response details.
- Added and completed
- - Balance API.
- - Payout API.
- - Refund API.
- - Mandate API.
- Error
- - Renamed members of `Types`.
- Documentation improvements and other changes...

## 0.2.2 (01/11/2023)
- Added "Debug" attribute for some structs.
- PaymentIntent
- - Added missing Response data `payment_method_options`, `metadata`, `processing` and `setup_future_usage`.
- Error
- - Added missing Response data `param` and `payment_intent`.
- Some bug fixes, documentation improvements and other changes...

## 0.2.1 (01/11/2023)
- Added debug option.
- PaymentIntent:
- - Added `get()` function.
- - Fixed wrong request for `RETRIEVE`.
- - Added `LIST` and `UPDATE` request.
- - Added check if the right method was used.
- - `send()` function returns now `Result<crate::payment_intent::Response, (String, Option<crate::error::Info>)>`
- Other changes...

## 0.2.0 (01/11/2023)
- Complete restructuring.
- PaymentIntent
- - Added more response details.
- - Added more documentation.
- Added more documentation.
- Other changes...

## 0.1.1 (01/09/2023)
- Mutex removed from SECRET_KEY.
- Function go() renamed to send() [payment_intent::___::Info].
- Documentation updated.
- Other changes...

## 0.1.0 (01/09/2023)