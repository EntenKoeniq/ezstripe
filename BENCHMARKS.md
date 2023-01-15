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
    <span> | </span>
    <a href="https://github.com/EntenKoeniq/ezstripe/blob/main/BENCHMARKS.md">
      Benchmarks
    </a>
  </h4>
</div>

# Benchmarks
Here are simple benchmark results on other dependencies with source code so you can try it yourself.

<b>Description:</b> 6 threads and each thread created a payment intent 20 times.
<br>
<b>Build:</b> Release (default).

<details>
  <summary>Source code</summary>

```toml
# Config.toml
[dependencies]
tokio = { version = "1.24.1", features = ["full"] }
ezstripe = "0.4.0"
```

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
| First run | 6.26s | 6.39s | 6.33s | 6.56s | 6.51s | 6.22s | 6.38s |
| Second run | 6.39s | 6.47s | 6.33s | 6.44s | 6.29s | 6.39s | 6.39s |
| Third run | 6.40s | 6.42s | 6.34s | 6.44s | 6.31s | 6.29s | 6.37s |

<details>
  <summary>Source code</summary>

```toml
# Config.toml
[dependencies]
tokio = { version = "1.24.1", features = ["full"] }
async-stripe = { version = "0.15.0", features = ["runtime-async-std-surf"] }
```
  
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
        let mut create_intent = CreatePaymentIntent::new(1500, Currency::EUR);
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
| First run | 6.57s | 6.37s | 6.56s | 6.31s | 6.65s | 6.39s | 6.48s |
| Second run | 6.53s | 6.63s | 6.42s | 6.36s | 6.75s | 6.58s | 6.55s |
| Third run | 6.58s | 6.55s | 6.52s | 6.43s | 6.46s | 6.62s | 6.53s |

Performance result
| [ezstripe 0.4.0](https://crates.io/crates/ezstripe) | [async-stripe 0.15.0](https://crates.io/crates/async-stripe) |
| --- | --- |
| 100% | 97.85% |