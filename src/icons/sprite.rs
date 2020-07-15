use crate::physics::mechanics::Mechanics as Mechanics;

pub struct Sprite {
  pub width: f32,
  pub height: f32,
  pub sprite_box: [f32; 8],
  pub box_indicies: [u16; 6],
  pub index_count: i32,
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
        sprite_box: [ 0., 1., 0., 0., 1., 1., 1., 0., ],
        box_indicies: [0, 1, 2, 2, 1, 3],
        index_count: 0,
        mechanics: Mechanics::new(position, velocity)
      }
  }

  pub fn set_index_count(&mut self, index_count_value: i32) {
    self.index_count = index_count_value;
  }

  pub fn verticies_location(&self) -> u32 {
    self.sprite_box.as_ptr() as u32 / 4
  }

  pub fn indices_location(&self) -> u32 {
    self.box_indicies.as_ptr() as u32 / 2
  }
}