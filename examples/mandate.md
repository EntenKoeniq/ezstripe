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
    <a href="https://github.com/xEntenKoeniqx/ezstripe/tree/main/examples">
      Examples
    </a>
  </h4>
</div>

### Example
```toml
# Cargo.toml
[dependencies]
tokio = { version = "1.24.1", features = ["full"] }
ezstripe = "0.2.2"
```

```Rust
#[tokio::main]
async fn main() {
  // Enable debug to show possible errors in our console
  unsafe {
    ezstripe::set_debug(true);
  };

  let client = ezstripe::Client {
    secret_key: "YOUR_SECRET_KEY".to_string()
  };

  let stripe_response = client.retrieve_mandate("ID_OF_MANDATE".to_string()).get().await;
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