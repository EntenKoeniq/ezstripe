impl Client {
  pub fn retrieve_balance(&self) -> crate::balance::Info {
    crate::balance::Info {
      r#type: crate::balance::Types::RETRIEVE,
      secret_key: self.secret_key.clone()
    }
  }
}