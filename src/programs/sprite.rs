use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use super::super::common_functions as cf;

pub struct Sprite {
  program: WebGlProgram,
  u_color: WebGlUniformLocation,
  index_count: i32,
  sprite_box_buffer: WebGlBuffer,
  u_opacity: WebGlUniformLocation,
  u_transform: WebGlUniformLocation,
  width: f32,
  height: f32
}

impl Sprite {
  pub fn new(gl: &WebGlRenderingContext, width: u8, height: u8) -> Self {
      let program = cf::link_program(
        &gl,
        super::super::shaders::vertex::sprite::SHADER,
        super::super::shaders::fragment::sprite::SHADER
      ).unwrap();

      let sprite_box: [f32; 8] = [
        0., 1.,
        0., 0.,
        1., 1.,
        1., 0.,
      ];

      let box_memory_buffer = wasm_bindgen::memory()
          .dyn_into::<WebAssembly::Memory>()
          .unwrap()
          .buffer();
      let verticies_location = sprite_box.as_ptr() as u32 / 4;
      let vert_array = js_sys::Float32Array::new(&box_memory_buffer).subarray(
        verticies_location,
        verticies_location + sprite_box.len() as u32,
      );
      let sprite_box_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
      gl.bind_buffer(GL::ARRAY_BUFFER, Some(&sprite_box_buffer));
      gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

      // TODO: Add texture coordinates;

      let box_indicies: [u16; 6] = [0, 1, 2, 2, 1, 3];

      let indices_memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
      let indices_location = box_indicies.as_ptr() as u32 / 2;
      let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
          indices_location,
          indices_location + box_indicies.len() as u32
      );
      let box_indicies_buffer = gl.create_buffer().unwrap();
      gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&box_indicies_buffer));
      gl.buffer_data_with_array_buffer_view(
          GL::ELEMENT_ARRAY_BUFFER,
          &indices_array,
          GL::STATIC_DRAW,
      );

      Self {
        u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
        index_count: indices_array.length() as i32,
        u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
        u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
        sprite_box_buffer: sprite_box_buffer,
        program: program,
        width: width as f32,
        height: height as f32
      }
  }

  pub fn render(
    &self,
    gl: &WebGlRenderingContext,
    bottom: f32,
    _top: f32,
    left: f32,
    _right: f32,
    canvas_height: f32,
    canvas_width: f32
  ) {
    gl.use_program(Some(&self.program));
    
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.sprite_box_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    gl.uniform4f(
      Some(&self.u_color),
      1.0,
      1.0,
      1.0,
      1.0
    );

    gl.uniform1f(Some(&self.u_opacity), 0.9);

    let translation_mat = cf::translation_matrix(
        2. * left / canvas_width - 1.,
        2. * bottom / canvas_height - 1.,
        0.,
    );

    let scale_mat = cf::scaling_matrix(
        0.01 + self.width / canvas_width,
        0.01 + self.height / canvas_height,
        0.,
    );

    let transform_mat = cf::mult_matrix_4(scale_mat, translation_mat);
    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat);

    gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
  }
}