<h1 align="center">ezstripe 💳</h1>
<div align="center">
 <strong>
   A Stripe-SDK for Rustlang
 </strong>
 <p>Use ezstripe to easily communicate with Stripe's API.</p>
</div>

<div align="center">

  [![CRATESIO]][CRATESIO_URL] [![DOCS]][DOCS_URL] [![EXAMPLES]][EXAMPLES_URL] [![CHANGELOG]][CHANGELOG_URL] [![BENCHMARKS]][BENCHMARKS_URL]
 
  <a href="https://discord.gg/VjQQZRU22F"><img src="https://discordapp.com/api/guilds/1065101892914511953/widget.png?style=shield" alt="Your our discord" /></a>
  
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
ezstripe = { path = "./../ezstripe-master" }
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
        "payment_method_types[]" => "sofort",
        "capture_method" => "automatic",
        "shipping[name]" => "Your Name",
        "shipping[address][city]" => "Test"
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


| [ezstripe 0.6.0](https://crates.io/crates/ezstripe) | #0 | #1 | #2 | #3 | #4 | #5 | AVG |
| ------- | --- | --- | --- | --- | --- | --- | --- |
| First run | 7.25s | 7.43s | 7.20s | 7.33s | 7.03s | 7.21s | 7.24s |
| Second run | 7.10s | 7.13s | 7.08s | 7.14s | 6.99s | 7.13s | 7.10s |
| Third run | 7.13s | 7.13s | 7.35s | 7.21s | 7.01s | 7.05s | 7.15s |

<details>
  <summary>Source code</summary>

```toml
# Config.toml
[dependencies]
tokio = { version = "1.24.1", features = ["full"] }
async-stripe = { path = "./../async-stripe-master", features = ["runtime-async-std-surf"] }
```
  
```Rust
use stripe::{
  Client,
  CreatePaymentIntent,
  Currency,
  PaymentIntent,
  PaymentIntentCaptureMethod,
  CreatePaymentIntentShipping,
  CreatePaymentIntentShippingAddress
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
        create_intent.payment_method_types = Some(vec![
          "card".to_string(),
          "sofort".to_string()
        ]);
        create_intent.capture_method = Some(PaymentIntentCaptureMethod::Automatic);
        create_intent.shipping = Some(CreatePaymentIntentShipping {
          address: CreatePaymentIntentShippingAddress {
            city: Some("Test".to_string()),
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None
          },
          carrier: None,
          name: "Your Name".to_string(),
          phone: None,
          tracking_number: None
        });

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

| [async-stripe 0.16.0](https://crates.io/crates/async-stripe) | #0 | #1 | #2 | #3 | #4 | #5 | AVG |
| ------- | --- | --- | --- | --- | --- | --- | --- |
| First run | 7.46 | 7.40s | 7.20s | 7.49s | 7.06s | 7.25s | 7.31s |
| Second run | 7.23s | 7.15s | 7.25s | 7.05s | 7.05s | 7.09s | 7.14s |
| Third run | 7.45s | 7.30s | 7.17s | 7.16s | 7.63s | 7.43s | 7.36s |

Performance result
| [ezstripe 0.6.0](https://crates.io/crates/ezstripe) | [async-stripe 0.16.0](https://crates.io/crates/async-stripe) |
| --- | --- |
| 100% | 98.34% |

Build time
| [ezstripe 0.6.0](https://crates.io/crates/ezstripe) | [async-stripe 0.16.0](https://crates.io/crates/async-stripe) |
| --- | --- |
| 43s | 4m 19s |

 File size comparison
| [ezstripe 0.6.0](https://crates.io/crates/ezstripe) | [async-stripe 0.16.0](https://crates.io/crates/async-stripe) |
| --- | --- |
| 4.546 KB | 11.661 KB |

[CRATESIO]: https://img.shields.io/badge/crates.io-ezstripe-B7410E?style=flat-square&logo=rust
[CRATESIO_URL]: https://crates.io/crates/ezstripe
[DOCS]: https://img.shields.io/badge/docs-latest-343434?style=flat-square&logo=read-the-docs&logoColor=fff
[DOCS_URL]: https://docs.rs/ezstripe/latest/ezstripe/
[EXAMPLES]: https://img.shields.io/badge/examples-latest-343434?style=flat-square&logo=bookstack&logoColor=fff
[EXAMPLES_URL]: https://github.com/EntenKoeniq/ezstripe/tree/main/examples
[CHANGELOG]: https://img.shields.io/badge/changelog-latest-343434?style=flat-square&logo=react-hook-form&logoColor=fff
[CHANGELOG_URL]: https://github.com/EntenKoeniq/ezstripe/blob/main/CHANGELOG.md
[BENCHMARKS]: https://img.shields.io/badge/benchmarks-0.6.0-ffd73c?style=flat-square&logo=speedtest
[BENCHMARKS_URL]: https://github.com/EntenKoeniq/ezstripe/blob/main/BENCHMARKS.md