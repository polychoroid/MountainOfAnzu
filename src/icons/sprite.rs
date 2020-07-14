use crate::physics::mechanics::Mechanics as Mechanics;

pub struct Sprite {
  pub width: f32,
  pub height: f32,
  pub mechanics: Mechanics,
}

impl Sprite {
  pub fn new(
      width: u8,
      height: u8,
      position: Vec<f32>,
      velocity: Vec<f32>) -> Self {

      Self {        
        width: width as f32,
        height: height as f32,
        mechanics: Mechanics::new(position, velocity)
      }
  }
}