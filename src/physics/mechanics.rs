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

  pub fn step(&mut self) {
    self.position[0] += self.velocity[0];
    self.position[1] += self.velocity[1];
  }

  pub fn gravity(&mut self) {
    self.velocity[0] += -0.000059;
  }

  pub fn edge_bounce(&mut self) {
    // TODO: This is so procedural it hurts my feelings.
    if self.position[0] >= 1.0 {
      self.velocity[0] = self.velocity[0] * -0.799;
      self.position[0] = 1.0;
    }

    if self.position[0] <= -1.0 {
      self.velocity[0] = self.velocity[0] * -0.799;
      self.position[0] = -1.0;
    }

    if self.position[1] >= 1.0 {
      self.velocity[1] = self.velocity[1] * -0.799;
      self.position[1] = 1.0;
    }

    if self.position[1] <= -1.0 {
      self.velocity[1] = self.velocity[1] * -0.799;
      self.position[1] = -1.0;
    }
  }

  pub fn edge_to_center(&mut self) {
    // TODO: This is so procedural it hurts my feelings.
    if self.position[0] >= 1.0 || self.position[0] <= -1.0 ||self.position[1] >= 1.0 || self.position[1] <= -1.0 {
      self.position[0] = 0.;
      self.position[1] = 0.;
    }
  }
}
