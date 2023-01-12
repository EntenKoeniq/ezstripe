impl Client {
  pub fn retrieve_balance(&self) -> crate::balance::Info {
    crate::balance::Info {
      secret_key: self.secret_key.clone()
    }
  }
}