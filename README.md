<h1 align="center">ezstripe</h1>
<div align="center">
 <strong>
   A easy SDK for Stripe (Rust)
 </strong>
</div>

<div align="center">
  <h4>
    <a href="#install">
      Install
    </a>
    <span> | </span>
    <a href="#example">
      Example
    </a>
  </h4>
</div>

## Install
Add ezstripe to your project:
```toml
# Cargo.toml
[dependencies]
ezstripe = "*" # Latest version
```

# Example
```Rust
#[macro_use]
extern crate ezstripe;

#[tokio::main]
async fn main() {
  // Be sure to set your secret key before making a request
  unsafe {
    ezstripe::set_secret("SECRET_KEY").ok();
  };
  
  let stripe_order = ezstripe::payment_intent::create::Info {
    body: ezbody!(
      "amount" => 1500,
      "currency" => "eur",
      "payment_method_types[]" => "card",
      "capture_method" => "manual"
    )
  };

  let stripe_order_go = match stripe_order.go().await {
    Ok(r) => r,
    Err(e) => {
      if let Some(r) = e {
        println!("{} | {} | {}", r.r#type.original_str(), r.code.original_str(), r.message);
      } else { // Should never happen!
        println!("Unknown error!");
      }
      std::process::exit(1);
    }
  };
  
  println!("Created: {}", stripe_order_go.id);
}
```