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

> **Warning** <br>
> Up to version `1.0.0` a lot of the structure can change with each new version! <br>
> Therefore, with every new version, pay attention to the changes in the changelogs on [Github](https://github.com/xEntenKoeniqx/ezstripe)!

### Example
```toml
# Cargo.toml
[dependencies]
tokio = { version = "1.24.1", features = ["full"] }
ezstripe = "0.3.0"
```

```Rust
// Required to use the `ezbody!` macro
#[macro_use] extern crate ezstripe;

#[tokio::main]
async fn main() {
  // Enable debug to show possible errors in our console
  unsafe {
    ezstripe::set_debug(true);
  };

  let client = ezstripe::Client {
    secret_key: "YOUR_SECRET_KEY".to_string()
  };
  
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
      println!("{}: {} | {} | {}", e_msg, r.r#type.original_str(), r.code.original_str(), r.message);
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