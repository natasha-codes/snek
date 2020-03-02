mod snek;

fn main() {
  let mut d = snek::driver::Driver::new();

  d.drive()
    .map_err(|err| eprintln!("Driver returned with an error: {:?}", err))
    .unwrap();
}
