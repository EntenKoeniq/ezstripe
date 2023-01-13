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
  </h4>
</div>

### Example
```toml
# Cargo.toml
[dependencies]
tokio = { version = "1.24.1", features = ["full"] }
ezstripe = "0.3.2"
env_logger = "0.10.0"
```

```Rust
// Required to use the `ezbody!` macro
#[macro_use] extern crate ezstripe;

#[tokio::main]
async fn main() {
  // To show possible errors (recommended for development)
  env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "debug"));

  let client = ezstripe::Client {
    secret_key: "YOUR_SECRET_KEY".to_string()
  };

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
