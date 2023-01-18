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

### Example
```toml
# Cargo.toml
[dependencies]
tokio = { version = "1.24.1", features = ["full"] }
ezstripe = "0.6.0"
env_logger = "0.10.0"
```

```Rust
// Required to use the `ezbody!` macro
#[macro_use] extern crate ezstripe;

#[tokio::main]
async fn main() {
  // To show possible errors (recommended for development)
  env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "debug"));

  let client = ezstripe::Client::new("SECRET_KEY");

  // Returns: String("amount=1500;currency=eur;")
  let stripe_body = ezbody!(
    "charge" => "ID..."
  );

  let stripe_response = client.create_refund(stripe_body).send().await;
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
  
  println!("{:?}", stripe_result);
}
```

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