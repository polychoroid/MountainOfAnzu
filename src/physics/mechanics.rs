pub struct Mechanics {
  pub position: Vec<f32>,
  pub velocity: Vec<f32>,
}

impl Mechanics {
  pub fn new(position: Vec<f32>, velocity: Vec<f32>) -> Self {
    Mechanics {
      position: position,
      velocity: velocity,
    }
  }

  pub fn edge_bounce(&mut self) {
    // TODO: This is so procedural it hurts my feelings.
    if self.position[0] <= 1.0 && self.position[0] >= -1.0 {
      self.position[0] += self.velocity[0];
    }

    if self.position[0] >= 1.0 {
      self.velocity[0] = self.velocity[0] * -1.0;
      self.position[0] = 1.0;
    }

    if self.position[0] <= -1.0 {
      self.velocity[0] = self.velocity[0] * -1.0;
      self.position[0] = -1.0;
    }

    if self.position[1] <= 1.0 && self.position[1] >= -1.0 {
      self.position[1] += self.velocity[1];
    }

    if self.position[1] >= 1.0 {
      self.velocity[1] = self.velocity[1] * -1.0;
      self.position[1] = 1.0;
    }

    if self.position[1] <= -1.0 {
      self.velocity[1] = self.velocity[1] * -1.0;
      self.position[1] = -1.0;
    }
  }
}
