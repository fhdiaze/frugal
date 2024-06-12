struct Menu {
  action: String,
}

pub fn handle() -> Menu {
  Menu {
    action: String::from("price.scale"),
  }
}
