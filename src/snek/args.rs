/// Play the classic game snake! 🐍
#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Nathan Shelly & Sasha Weiss")]
pub struct Args {
  /// Sets the amount of food on the board at any given time
  #[clap(long = "food-count")]
  food_count: Option<i32>,
}
