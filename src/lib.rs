use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod common_functions;
mod gl_setup;
mod programs;
mod shaders;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub struct GameClient {
  gl: WebGlRenderingContext,
  program_sprite: programs::Sprite,
  background: programs::Color2DGradient,
}

#[wasm_bindgen]
impl GameClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    log("New game client requested.");
    console_error_panic_hook::set_once();
    let gl = gl_setup::initialize_webgl_context().unwrap();

    Self {
      program_sprite: programs::Sprite::new(&gl, 32, 32),
      background: programs::Color2DGradient::new(&gl),
      gl: gl,
    }
  }

  pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
    app_state::update_dynamic_data(time, height, width);
    Ok(())
  }

  pub fn render(&self)
  {
    self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    let curr_state = app_state::get_curr_state();

    self.background.render(
      &self.gl
    );

     self.program_sprite.render(
      &self.gl,
      curr_state.control_bottom,
      curr_state.control_top,
      curr_state.control_left,
      curr_state.control_right,
      curr_state.canvas_height,
      curr_state.canvas_width
    );      

  }
}