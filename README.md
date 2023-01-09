<h1 align="center">ezstripe 💳</h1>
<div align="center">
 <strong>
   A ezStripe SDK for Rustlang
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
  </h4>
</div>

# Usage
### Installation
```toml
# Cargo.toml
[dependencies]
ezstripe = "*" # Latest version
```
or
`cargo add ezstripe`

### Example
```toml
# Cargo.toml
[dependencies]
tokio = { version = "1.24.1", features = ["full"] }
ezstripe = "0.1.0"
```

```Rust
// Required to use the `ezbody!` macro
#[macro_use] extern crate ezstripe;

#[tokio::main]
async fn main() {
  // Be sure to set your secret key before making a request
  unsafe {
    ezstripe::set_secret("SECRET_KEY");
  };
  
  let stripe_order = ezstripe::payment_intent::create::Info {
    body: ezbody!( // Returns "amount=1500;currency=eur;payment_method_types[]=card;capture_method=manual;"
      "amount" => 1500,
      "currency" => "eur",
      "payment_method_types[]" => "card",
      "capture_method" => "manual"
    )
  };
  
  let stripe_order_res = match stripe_order.send().await {
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
  
  println!("Created: {}", stripe_order_res.id);
}
```