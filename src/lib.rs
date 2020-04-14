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
  program_sprites: Vec<programs::Sprite>,
  background: programs::Color2DGradient,
}

#[wasm_bindgen]
impl GameClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    log("New game client requested.");
    console_error_panic_hook::set_once();
    let gl = gl_setup::initialize_webgl_context().unwrap();

    let sprites = vec![
      programs::Sprite::new(&gl, 32, 32, vec![1., 0.], vec![-0.012, -0.012]),
      programs::Sprite::new(&gl, 64, 32, vec![-1., 0.], vec![0.015, 0.013]),
      programs::Sprite::new(&gl, 32, 32, vec![1., 0.], vec![-0.017, -0.015]),
      programs::Sprite::new(&gl, 32, 32, vec![-1., 0.], vec![0.019, 0.016]),
      programs::Sprite::new(&gl, 32, 16, vec![1., 0.], vec![-0.018, -0.014]),
      programs::Sprite::new(&gl, 32, 32, vec![-1., 0.], vec![0.017, 0.013]),
      programs::Sprite::new(&gl, 32, 32, vec![1., 0.], vec![-0.016, -0.012]),
      programs::Sprite::new(&gl, 32, 32, vec![-1., 0.], vec![0.015, 0.01]),
      programs::Sprite::new(&gl, 32, 32, vec![1., 0.], vec![-0.013, -0.011]),
      programs::Sprite::new(&gl, 64, 64, vec![-1., 0.], vec![0.012, 0.019]),
      programs::Sprite::new(&gl, 32, 32, vec![1., 0.], vec![-0.011, -0.018]),
      programs::Sprite::new(&gl, 32, 32, vec![-1., 0.], vec![0.01, 0.014]),
      programs::Sprite::new(&gl, 16, 16, vec![1., 0.], vec![-0.015, -0.014]),
      programs::Sprite::new(&gl, 32, 32, vec![-1., 0.], vec![-0.017, 0.016]),
      programs::Sprite::new(&gl, 16, 16, vec![1., 0.], vec![0.018, -0.019]),
      programs::Sprite::new(&gl, 32, 32, vec![-1., 0.], vec![-0.019, 0.012]),
      programs::Sprite::new(&gl, 32, 32, vec![1., 0.], vec![0.017, -0.013]),
      programs::Sprite::new(&gl, 16, 32, vec![-1., 0.], vec![-0.014, 0.014]),
      programs::Sprite::new(&gl, 32, 32, vec![1., 0.], vec![0.015, -0.016]),
      programs::Sprite::new(&gl, 32, 64, vec![-1., 0.], vec![-0.013, 0.015]),
      programs::Sprite::new(&gl, 32, 32, vec![1., 0.], vec![-0.017, 0.013]),
      programs::Sprite::new(&gl, 32, 32, vec![-1., 0.], vec![0.016, -0.012]),
      programs::Sprite::new(&gl, 32, 32, vec![1., 0.], vec![-0.015, 0.01]),
      programs::Sprite::new(&gl, 32, 32, vec![-1., 0.], vec![0.013, -0.011]),
      programs::Sprite::new(&gl, 64, 64, vec![1., 0.], vec![-0.012, 0.019]),
      programs::Sprite::new(&gl, 32, 32, vec![-1., 0.], vec![0.011, -0.018]),
      programs::Sprite::new(&gl, 32, 32, vec![1., 0.], vec![-0.01, 0.014]),
      programs::Sprite::new(&gl, 16, 16, vec![-1., 0.], vec![0.015, -0.014]),
      programs::Sprite::new(&gl, 32, 32, vec![1., 0.], vec![-0.017, 0.016]),
      programs::Sprite::new(&gl, 16, 16, vec![-1., 0.], vec![0.018, -0.019]),
      programs::Sprite::new(&gl, 32, 32, vec![1., 0.], vec![-0.019, 0.012]),
      programs::Sprite::new(&gl, 32, 32, vec![-1., 0.], vec![0.017, -0.013]),
      programs::Sprite::new(&gl, 16, 32, vec![1., 0.], vec![-0.014, 0.014]),
      programs::Sprite::new(&gl, 32, 32, vec![-1., 0.], vec![0.015, -0.016]),
      programs::Sprite::new(&gl, 32, 64, vec![1., 0.], vec![-0.013, 0.015]),
      ];

    Self {
      program_sprites: sprites,
      background: programs::Color2DGradient::new(&gl),
      gl: gl,
    }
  }

  pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
    app_state::update_dynamic_data(time, height, width);
    Ok(())
  }

  pub fn render(&mut self)
  {
    self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    let curr_state = app_state::get_curr_state();

    self.background.render(
      &self.gl
    );

    for index in 0..self.program_sprites.len()
    {
      self.program_sprites[index].render(
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
}