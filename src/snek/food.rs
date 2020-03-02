use rand::{
  distributions::{Distribution, Standard},
  Rng,
};

#[derive(Debug, Copy, Clone)]
pub(crate) enum Food {
  Mouse,
  Cherry,
  Cake,
}

// ref - https://stackoverflow.com/a/48491021
impl Distribution<Food> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Food {
    match rng.gen_range(0, 3) {
      0 => Food::Mouse,
      1 => Food::Cherry,
      _ => Food::Cake,
    }
  }
}
