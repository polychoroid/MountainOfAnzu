use super::super::common_functions as cf;
use crate::display::Canvas;
use crate::icons::sprite::Sprite;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct BufferedSprite {
  buffer: WebGlBuffer,
  pub sprite: Sprite,
  program: WebGlProgram,
  u_color: WebGlUniformLocation,
  u_opacity: WebGlUniformLocation,
  u_transform: WebGlUniformLocation
}

pub struct WasmWebglScene {
  gl: WebGlRenderingContext,
  buffered_sprites: Vec<BufferedSprite>
}

impl WasmWebglScene {
  pub fn new(gl: WebGlRenderingContext, sprites: Vec<Sprite>) -> Self {
    let buffered_sprites = Self::buffer_sprites(&gl, sprites);

    WasmWebglScene {
      buffered_sprites: buffered_sprites,
      gl: gl,
    }
  }

  pub fn clear(&self) {
    self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
  }

  fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
  ) -> Result<WebGlShader, String> {
    let shader = gl
      .create_shader(shader_type)
      .ok_or_else(|| String::from("Error creating shader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
      .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
      .as_bool()
      .unwrap_or(false)
    {
      Ok(shader)
    } else {
      Err(
        gl.get_shader_info_log(&shader)
          .unwrap_or_else(|| String::from("Unable to get shader info log")),
      )
    }
  }

  fn link_program(
    gl: &WebGlRenderingContext,
    vert_source: &str,
    frag_source: &str,
  ) -> Result<WebGlProgram, String> {
    let program = gl
      .create_program()
      .ok_or_else(|| String::from("Error creating program"))?;

    let vert_shader =
      WasmWebglScene::compile_shader(&gl, GL::VERTEX_SHADER, vert_source).unwrap();

    let frag_shader =
      WasmWebglScene::compile_shader(&gl, GL::FRAGMENT_SHADER, frag_source).unwrap();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl
      .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
      .as_bool()
      .unwrap_or(false)
    {
      Ok(program)
    } else {
      Err(
        gl.get_program_info_log(&program)
          .unwrap_or_else(|| String::from("Error creating program object")),
      )
    }
  }

  fn buffer_sprites(gl: &WebGlRenderingContext, mut sprites: Vec<Sprite>) -> Vec<BufferedSprite> {
    let mut buffered_sprites = Vec::new();

    while !sprites.is_empty() {
      match sprites.pop() {
        Some(mut sprite) => {
          let program = WasmWebglScene::link_program(
            &gl,
            super::super::display::shaders::vertex::sprite::SHADER,
            super::super::display::shaders::fragment::sprite::SHADER,
          )
          .unwrap();

          let box_memory_buffer = wasm_bindgen::memory()
          .dyn_into::<WebAssembly::Memory>()
          .unwrap()
          .buffer();
    
        let vert_array = js_sys::Float32Array::new(&box_memory_buffer).subarray(
          sprite.verticies_location(),
          sprite.verticies_location() + sprite.sprite_box.len() as u32,
        );
    
        let sprite_box_buffer = gl
          .create_buffer()
          .ok_or("failed to create buffer")
          .unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&sprite_box_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
    
        let indices_memory_buffer = wasm_bindgen::memory()
          .dyn_into::<WebAssembly::Memory>()
          .unwrap()
          .buffer();
    
        let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
          sprite.indices_location(),
          sprite.indices_location() + sprite.box_indicies.len() as u32,
        );
    
        sprite.set_index_count(indices_array.length() as i32);
    
        let box_indicies_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&box_indicies_buffer));
        gl.buffer_data_with_array_buffer_view(
          GL::ELEMENT_ARRAY_BUFFER,
          &indices_array,
          GL::STATIC_DRAW,
        );
    
        buffered_sprites.push(BufferedSprite {
          sprite: sprite,
          buffer: sprite_box_buffer,          
          u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
          u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
          u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
          program: program,
        })
        }
        None => panic!()
      }      
    }
    buffered_sprites
  }

  pub fn render_scene(&mut self, canvas: &Canvas) {
    self.render_background();
    for index in 0..self.buffered_sprites.len() {     
      self.render_sprite(&canvas, &self.buffered_sprites[index]);
      self.buffered_sprites[index].sprite.mechanics.gravity();
      self.buffered_sprites[index].sprite.mechanics.step();
      self.buffered_sprites[index].sprite.mechanics.edge_bounce();
    }
  }

  fn render_background(&self) {
    let background = WasmWebglScene::link_program(
      &self.gl,
      super::super::display::shaders::vertex::color_2d_gradient::SHADER,
      super::super::display::shaders::fragment::color_2d_gradient::SHADER,
    )
    .unwrap();

    self.gl.use_program(Some(&background));

    let indices_rect: [u16; 6] = [0, 1, 2, 2, 1, 3];

    let indices_memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
    let indices_location = indices_rect.as_ptr() as u32 / 2;
    let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
      indices_location,
      indices_location + indices_rect.len() as u32,
    );
    let buffer_indices = self.gl.create_buffer().unwrap();
    self
      .gl
      .bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices));
    self.gl.buffer_data_with_array_buffer_view(
      GL::ELEMENT_ARRAY_BUFFER,
      &indices_array,
      GL::STATIC_DRAW,
    );

    let verticies_rect: [f32; 8] = [0., 1., 0., 0., 1., 1., 1., 0.];

    let u_opacity = self
      .gl
      .get_uniform_location(&background, "uOpacity")
      .unwrap();
    let u_transform = self
      .gl
      .get_uniform_location(&background, "uTransform")
      .unwrap();
    let memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
    let verticies_location = verticies_rect.as_ptr() as u32 / 4;
    let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
      verticies_location,
      verticies_location + verticies_rect.len() as u32,
    );

    let rect_vertice_buffer = self
      .gl
      .create_buffer()
      .ok_or("failed to create buffer")
      .unwrap();
    self
      .gl
      .bind_buffer(GL::ARRAY_BUFFER, Some(&rect_vertice_buffer));
    self
      .gl
      .buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

    self
      .gl
      .bind_buffer(GL::ARRAY_BUFFER, Some(&rect_vertice_buffer));
    self
      .gl
      .vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    self.gl.enable_vertex_attrib_array(0);

    let color_buffer = self
      .gl
      .create_buffer()
      .ok_or("failed to create color buffer")
      .unwrap();

    self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&color_buffer));
    self
      .gl
      .vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, 0, 0);
    self.gl.enable_vertex_attrib_array(1);

    let colors: [f32; 16] = [
      1., 0., 0., 0.25, 0., 1., 0., 0.25, 0., 0., 1., 0.25, 1., 1., 1., 0.25,
    ];

    let colors_memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();

    let color_vals_location = colors.as_ptr() as u32 / 4;
    let color_vals_array = js_sys::Float32Array::new(&colors_memory_buffer).subarray(
      color_vals_location,
      color_vals_location + colors.len() as u32,
    );
    self.gl.buffer_data_with_array_buffer_view(
      GL::ARRAY_BUFFER,
      &color_vals_array,
      GL::DYNAMIC_DRAW,
    );

    self.gl.uniform1f(Some(&u_opacity), 1.);

    let translation_mat = cf::translation_matrix(-1., -1., -1.);

    let scale_mat = cf::scaling_matrix(2., 2., 0.);

    let transform_mat = cf::mult_matrix_4(scale_mat, translation_mat);
    self
      .gl
      .uniform_matrix4fv_with_f32_array(Some(&u_transform), false, &transform_mat);

    let index_count = indices_array.length() as i32;

    self
      .gl
      .draw_elements_with_i32(GL::TRIANGLES, index_count, GL::UNSIGNED_SHORT, 0);
  }

  // This function can be made to render any generic renderable data structure
  fn render_sprite(&self, canvas: &Canvas, sprite: &BufferedSprite) {
    self.gl.use_program(Some(&sprite.program));    

    self
      .gl
      .bind_buffer(GL::ARRAY_BUFFER, Some(&sprite.buffer));
    self
      .gl
      .vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    self.gl.enable_vertex_attrib_array(0);

    self.gl.uniform4f(Some(&sprite.u_color), 1.0, 1.0, 1.0, 1.0);
    self.gl.uniform1f(Some(&sprite.u_opacity), 1.);

    let translation_mat = cf::translation_matrix(
      sprite.sprite.mechanics.position[1],
      sprite.sprite.mechanics.position[0],
      0.,
    );

    let scale_mat = cf::scaling_matrix(
      sprite.sprite.width / canvas.width,
      sprite.sprite.height / canvas.height,
      0.,
    );

    let transform_mat = cf::mult_matrix_4(scale_mat, translation_mat);
    self
      .gl
      .uniform_matrix4fv_with_f32_array(Some(&sprite.u_transform), false, &transform_mat);

    self
      .gl
      .draw_elements_with_i32(GL::TRIANGLES, sprite.sprite.index_count, GL::UNSIGNED_SHORT, 0);
  }
}
