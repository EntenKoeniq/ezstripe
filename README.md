<h1 align="center">ezstripe 💳</h1>
<div align="center">
 <strong>
   A Stripe-SDK for Rustlang
 </strong>
 <p>Use ezstripe to easily communicate with Stripe's API.</p>
</div>

<div align="center">

  [![CRATESIO]][CRATESIO_URL] [![DOCS]][DOCS_URL] [![EXAMPLES]][EXAMPLES_URL] [![CHANGELOG]][CHANGELOG_URL] [![BENCHMARKS]][BENCHMARKS_URL]
  
</div>

> **Warning** <br>
> Up to version `1.0.0` a lot of the structure can change with each new version! <br>
> Therefore, pay attention to the changes in the [changelog on Github](https://github.com/EntenKoeniq/ezstripe/blob/main/CHANGELOG.md) with every new version!

# Usage
### Installation
```toml
# Cargo.toml
[dependencies]
ezstripe = "0.5.0"
```
or
`cargo add ezstripe`

### Features
All features are enabled by default, but you can only select the features you really need for your project.

```toml
# Cargo.toml
[dependencies]
ezstripe = { version = "0.5.0", default-features = false, features = ["payment_intent", "refund"] }
```

[Check all available features](https://github.com/EntenKoeniq/ezstripe/blob/main/Cargo.toml#L12..L27)

### Example
```toml
# Cargo.toml
[dependencies]
ezstripe = "0.5.0"
env_logger = "0.10.0" # Optional
```

```Rust
// Required to use the `ezbody!` macro
#[macro_use] extern crate ezstripe;

#[tokio::main]
async fn main() {
  // To show possible errors (recommended for development)
  env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "debug"));

  // We need a client to make requests
  let client = ezstripe::Client::new("SECRET_KEY");
  
  // Create a body for the request
  let stripe_body = ezbody!(
      "amount" => 1500,
      "currency" => "eur",
      "payment_method_types[]" => "card",
      "payment_method_types[]" => "sofort",
      "capture_method" => "automatic",
      "shipping[name]" => "Your Name",
      "shipping[address][city]" => "Test"
    );
  
  // Now send a request to Stripe's API
  let stripe_response = client.create_payment_intent(stripe_body).send().await;
  if let Err((e_msg, e_info)) = stripe_response {
    if let Some(r) = e_info {
      println!("{}: {} | {} | {}", e_msg, r.r#type, r.code, r.message);
    } else { // Such an error only occurs when a request to Stripe failed
      println!("{}", e_msg);
    }
    std::process::exit(1);
  }
  
  // No error, so let's unpack the answer
  let stripe_result = stripe_response.unwrap();
  
  // Print the unique ID from the created PaymentIntent
  println!("Created: {}", stripe_result.id);
}
```

# Status
A list of currently supported and planned features.

<b>[CORE RESOURCES](https://stripe.com/docs/api)</b>
- [X] Balance
- [ ] Balance Transactions
- [ ] Charges
- [ ] Customers
- [X] Disputes
- [ ] Events
- [ ] Files
- [ ] File Links
- [X] Mandates
- [X] PaymentIntents
- [ ] SetupIntents
- [ ] SetupAttempts
- [X] Payouts
- [X] Refunds
- [ ] Tokens

[CRATESIO]: https://img.shields.io/badge/crates.io-ezstripe-B7410E?style=flat-square&logo=rust
[CRATESIO_URL]: https://crates.io/crates/ezstripe
[DOCS]: https://img.shields.io/badge/docs-latest-343434?style=flat-square&logo=read-the-docs&logoColor=fff
[DOCS_URL]: https://docs.rs/ezstripe/latest/ezstripe/
[EXAMPLES]: https://img.shields.io/badge/examples-latest-343434?style=flat-square&logo=bookstack&logoColor=fff
[EXAMPLES_URL]: https://github.com/EntenKoeniq/ezstripe/tree/main/examples
[CHANGELOG]: https://img.shields.io/badge/changelog-latest-343434?style=flat-square&logo=react-hook-form&logoColor=fff
[CHANGELOG_URL]: https://github.com/EntenKoeniq/ezstripe/blob/main/CHANGELOG.md
[BENCHMARKS]: https://img.shields.io/badge/benchmarks-0.5.0-ffd73c?style=flat-square&logo=speedtest
[BENCHMARKS_URL]: https://github.com/EntenKoeniq/ezstripe/blob/main/BENCHMARKS.md