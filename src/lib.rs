use wasm_bindgen::prelude::*;
use rand_core::{RngCore, SeedableRng};
use rand_pcg::{Lcg128Xsl64};

#[macro_use]
extern crate lazy_static;

mod app_state;
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
  display: display::wasm_webgl::WasmWebglDisplay,
  sprites: Vec<icons::sprite::Sprite>,
}

#[wasm_bindgen]
impl GameClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    log("New game client requested.");
    console_error_panic_hook::set_once();
    let gl = gl_setup::initialize_webgl_context().unwrap();

    let display = display::wasm_webgl::WasmWebglDisplay::new(gl);

    let seed = [1,2,3,4, 5,6,7,8, 9,10,11,12, 13,14,15,16,17,18,19,20, 21,22,23,24, 25,26,27,28, 29,30,31,32];
    let mut rng = Lcg128Xsl64::from_seed(seed);

    let mut sprites = Vec::new();
    
    for _ in 1..250 {
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

    Self {
      sprites: sprites,
      display: display,
    }
  }

  pub fn negative_energy(number: u32) -> f32 {
    match number % 2 {
      1 | 2 => -1.,
      0 => 1.,
      _ => 0.
    }
  }

  pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
    app_state::update_dynamic_data(time, height, width);
    Ok(())
  }

  pub fn render(&mut self, height: f32, width: f32) {
    self.display.clear();

    let canvas = display::Canvas {
      height: height,
      width: width,
    };
    
    self.display.render_background();

    for index in 0..self.sprites.len() {
      self.sprites[index].mechanics.gravity();
      self.sprites[index].mechanics.step();
      self.sprites[index].mechanics.edge_bounce();

      self.display.render_sprite(&canvas, &self.sprites[index]);
    };
  }
}
