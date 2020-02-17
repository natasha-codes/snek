mod snek;

fn main() {
  let mut d = snek::driver::Driver::new();

  d.drive().expect("Driver returned with an error");
}
