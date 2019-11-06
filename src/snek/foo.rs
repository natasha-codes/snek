use tui;

pub fn foo() {
  println!("Hello world!");
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn panic() {
    foo();
    panic!();
  }
}
