pub fn foo() -> i32 {
  2 + 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dummy() {
    assert_eq!(foo(), 4)
  }
}
