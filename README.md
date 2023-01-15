<h1 align="center">ezstripe 💳</h1>
<div align="center">
 <strong>
   A Stripe-SDK for Rustlang
 </strong>
 <p>Use ezstripe to easily communicate with Stripe's API.</p>
</div>

<div align="center">
  <h4>
    <a href="https://crates.io/crates/ezstripe">
      Crate
    </a>
    <span> | </span>
    <a href="https://docs.rs/ezstripe/latest/ezstripe/">
      Docs
    </a>
    <span> | </span>
    <a href="https://github.com/EntenKoeniq/ezstripe/tree/main/examples">
      Examples
    </a>
    <span> | </span>
    <a href="https://github.com/EntenKoeniq/ezstripe/blob/main/CHANGELOG.md">
      Changelog
    </a>
  </h4>
</div>

> **Warning** <br>
> Up to version `1.0.0` a lot of the structure can change with each new version! <br>
> Therefore, pay attention to the changes in the [changelog on Github](https://github.com/EntenKoeniq/ezstripe/blob/main/CHANGELOG.md) with every new version!

# Usage
### Installation
```toml
# Cargo.toml
[dependencies]
ezstripe = "0.4.0"
```
or
`cargo add ezstripe`

### Features
All features are enabled by default, but you can only select the features you really need for your project.

```toml
# Cargo.toml
[dependencies]
ezstripe = { version = "0.4.0", default-features = false, features = ["payment_intent", "refund"] }
```

List of all available features:
- full
- - Default. Same as without "default-features = false"
- balance
- mandate
- payment_intent
- payout
- refund

### Example
```toml
# Cargo.toml
[dependencies]
ezstripe = "0.4.0"
env_logger = "0.10.0" # Optional
```

```Rust
// Required to use the `ezbody!` macro
#[macro_use] extern crate ezstripe;

#[tokio::main]
async fn main() {
  // To show possible errors (recommended for development)
  env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "debug"));

  let client = ezstripe::Client::new("SECRET_KEY");
  
  // Returns: String("amount=1500;currency=eur;payment_method_types[]=card;capture_method=manual;")
  let stripe_body = ezbody!(
      "amount" => 1500,
      "currency" => "eur",
      "payment_method_types[]" => "card",
      "capture_method" => "manual"
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
This SDK for Stripe is still in a very early version, which is why there can be many changes with each update.

The following is expected from version `1.0.0`: <br>
Complete and stable ...

- [ ] ... PaymentIntents (in progress)
- [ ] ... SetupIntents
- [ ] ... SetupAttempts
- [X] ... Payouts
- [X] ... Refunds
- [X] ... Mandates
- [ ] ... Disputes
- [ ] ... Charges
- [X] ... Balance

# Benchmarks
<b>Description:</b> 6 threads which created a payment intent 20 times <br>
<b>Build:</b> Release (default)

<details>
  <summary>Source code</summary>
  
```Rust
#[macro_use] extern crate ezstripe;

static mut CLIENT: Option<ezstripe::Client> = None;

fn create_thread(num: u16) {
  tokio::task::spawn(async move {
    let client = unsafe { CLIENT.as_ref().unwrap() };

    use std::time::Instant;
    let now = Instant::now();

    for _ in 0..20 {
      let stripe_body = ezbody!(
        "amount" => 1500,
        "currency" => "eur",
        "payment_method_types[]" => "card",
        "capture_method" => "manual"
      );

      let stripe_response = client.create_payment_intent(stripe_body).send().await.unwrap();
    }

    println!("THREAD {}: {:.2?}", num, now.elapsed());
  });
}

#[tokio::main]
async fn main() {
  unsafe {
    CLIENT = Some(ezstripe::Client::new("SECRET_KEY"));
  }

  for i in 0..6 {
    create_thread(i as u16);
  }

  loop {};
}
```
</details>


| [ezstripe](https://crates.io/crates/ezstripe) | #0 | #1 | #2 | #3 | #4 | #5 | AVG |
| ------- | --- | --- | --- | --- | --- | --- | --- |
| First run | 6.26s | 6.38s | 6.38s | 6.43s | 6.19s | 6.39s | 6.34s |
| Second run | 6.43s | 6.28s | 6.48s | 6.56s | 6.27s | 6.39s | 6.40s |
| Third run | 6.27s | 6.26s | 6.38s | 6.24s | 6.28s | 6.36s | 6.30s |

<details>
  <summary>Source code</summary>
  
```Rust
use stripe::{
  Client,
  CreatePaymentIntent,
  Currency,
  PaymentIntent,
  PaymentIntentCaptureMethod
};


static mut CLIENT: Option<Client> = None;

fn create_thread(num: u16) {
  tokio::task::spawn(async move {
    let client = unsafe { CLIENT.as_ref().unwrap() };

    use std::time::Instant;
    let now = Instant::now();

    for _ in 0..20 {
      let payment_intent = {
        let mut create_intent = CreatePaymentIntent::new(1000, Currency::USD);
        create_intent.amount = 1500;
        create_intent.currency = Currency::EUR;
        create_intent.payment_method_types = Some(vec!["card".to_string()]);
        create_intent.capture_method = Some(PaymentIntentCaptureMethod::Manual);

        PaymentIntent::create(&client, create_intent).await.unwrap()
      };
    }

    println!("THREAD {}: {:.2?}", num, now.elapsed());
  });
}

#[tokio::main]
async fn main() {
  unsafe {
    CLIENT = Some(Client::new("SECRET_KEY"));
  }

  for i in 0..6 {
    create_thread(i as u16);
  }

  loop {};
}
```
</details>

| [async-stripe](https://crates.io/crates/async-stripe) | #0 | #1 | #2 | #3 | #4 | #5 | AVG |
| ------- | --- | --- | --- | --- | --- | --- | --- |
| First run | 6.43s | 6.31s | 6.36s | 6.32s | 6.49s | 6.34s | 6.38s |
| Second run | 6.52s | 6.56s | 6.45s | 6.75s | 6.54s | 6.48s | 6.55s |
| Third run | 6.40s | 6.45s | 6.46s | 6.50s | 6.47s | 6.45s | 6.46s |

Performance result
| [ezstripe 0.4.0](https://crates.io/crates/ezstripe) | [async-stripe 0.15.0](https://crates.io/crates/async-stripe) |
| --- | --- |
| 100% | 98.29% |