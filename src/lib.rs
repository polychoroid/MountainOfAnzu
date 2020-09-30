use wasm_bindgen::prelude::*;
use rand_core::{RngCore, SeedableRng};
use rand_pcg::{Lcg128Xsl64};

mod common_functions;
mod display;
mod gl_setup;
mod icons;
mod physics;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub struct GameClient {
  display: display::wasm_webgl::WasmWebglScene
}

#[wasm_bindgen]
impl GameClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    log("New game client requested.");
    console_error_panic_hook::set_once();
    let gl = gl_setup::initialize_webgl_context().unwrap();    

    let seed = [1,2,3,4, 5,6,7,8, 9,10,11,12, 13,14,15,16,17,18,19,20, 21,22,23,24, 25,26,27,28, 29,30,31,32];
    let mut rng = Lcg128Xsl64::from_seed(seed);

    let mut sprites = Vec::new();
    
    for _ in 1..15 {
      let width = (rng.next_u32() % 96) as u8;
      let height = (rng.next_u32() % 96) as u8;

      let vy = GameClient::negative_energy(rng.next_u32()) * (rng.next_u32() % 1500) as f32 / 100000.;
      let vx = GameClient::negative_energy(rng.next_u32()) * (rng.next_u32() % 1500) as f32 / 100000.;
      let position = vec![0., 0.];
      let velocity = vec![vx, vy];
      sprites.push(icons::sprite::Sprite::new(
        width, height, position, velocity,
      ));
    }

    let display = display::wasm_webgl::WasmWebglScene::new(gl, sprites);

    Self {
      display: display
    }
  }

  pub fn negative_energy(number: u32) -> f32 {
    match number % 2 {
      1 | 2 => -1.,
      0 => 1.,
      _ => 0.
    }
  }

  pub fn render(&mut self, height: f32, width: f32) {
    self.display.clear();

    let canvas = display::Canvas {
      height: height,
      width: width,
    }; 
    
    self.display.render_scene(&canvas);
  }
}
