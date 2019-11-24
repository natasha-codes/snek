mod snek;

fn main() {
  let d = snek::driver::Driver::new();

  println!("{:?}", d.drive());
}
