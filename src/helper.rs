pub fn value_to_response<T>(val: serde_json::Value) -> Option<T> where T : serde::de::DeserializeOwned {
  match serde_json::from_value::<T>(val) {
    Ok(r) => return Some(r),
    Err(e) => {
      if crate::get_debug() {
        println!("[ezstripe]: {}Discovered errors! Send us this error so we can fix it (https://github.com/xEntenKoeniqx/ezstripe/issues){}", "\x1b[0;31m", "\x1b[0m");
        println!("{}", e);
      }
    }
  };

  None
}

pub fn value_to_response_list<T>(val: serde_json::Value) -> Option<Vec<T>>  where T : serde::de::DeserializeOwned {
  match serde_json::from_value::<Vec<T>>(val) {
    Ok(r) => return Some(r),
    Err(e) => {
      if crate::get_debug() {
        println!("[ezstripe]: {}Discovered errors! Send us this error so we can fix it (https://github.com/xEntenKoeniqx/ezstripe/issues){}", "\x1b[0;31m", "\x1b[0m");
        println!("{}", e);
      }
    }
  };

  None
}