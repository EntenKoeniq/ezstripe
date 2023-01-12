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

# Usage
### Installation
```toml
# Cargo.toml
[dependencies]
ezstripe = "0.2.2"
```
or
`cargo add ezstripe`

# Status
This SDK for Stripe is still in a very early version, which is why there can be many changes with each update.

The following is expected from version `1.0.0`: <br>
Complete and stable ...

- [ ] ... PaymentIntent
- [ ] ... SetupIntent
- [ ] ... SetupAttempts
- [X] ... Payouts
- [ ] ... Refunds
- [ ] ... Mandates
- [ ] ... Disputes
- [ ] ... Charges
- [X] ... Balance