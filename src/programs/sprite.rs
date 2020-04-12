use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use super::super::common_functions as cf;

pub struct Sprite {
  program: WebGlProgram,
  rect_vertice_buffer: WebGlBuffer,
  rect_vertice_array_length: usize,
  u_color: WebGlUniformLocation,
  u_opacity: WebGlUniformLocation,
  u_transform: WebGlUniformLocation
}

impl Sprite {
  pub fn new(gl: &WebGlRenderingContext) -> Self {
      let program = cf::link_program(
        &gl,
        super::super::shaders::vertex::sprite::SHADER,
        super::super::shaders::fragment::sprite::SHADER
      ).unwrap();

      let verticies_rect: [f32; 12] = [
        0., 1.,
        0., 0.,
        1., 1.,
        1., 1.,
        0., 0.,
        1., 0.,
      ];

      let memory_buffer = wasm_bindgen::memory()
          .dyn_into::<WebAssembly::Memory>()
          .unwrap()
          .buffer();
      let verticies_location = verticies_rect.as_ptr() as u32 / 4;
      let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
        verticies_location,
        verticies_location + verticies_rect.len() as u32,
      );
      let buffer_rect = gl.create_buffer().ok_or("failed to create buffer").unwrap();
      gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
      gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

      Self {
        u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
        u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
        u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
        rect_vertice_array_length: verticies_rect.len(),
        rect_vertice_buffer: buffer_rect,
        program: program,
      }
  }

  pub fn render(
    &self,
    gl: &WebGlRenderingContext,
    bottom: f32,
    top: f32,
    left: f32,
    right: f32,
    canvas_height: f32,
    canvas_width: f32
  ) {
    gl.use_program(Some(&self.program));
    
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    gl.uniform4f(
      Some(&self.u_color),
      0.75,
      0.,
      0.5,
      1.0
    );

    gl.uniform1f(Some(&self.u_opacity), 1.);

    let translation_mat = cf::translation_matrix(
        2. * left / canvas_width - 1.,
        2. * bottom / canvas_height - 1.,
        0.,
    );

    let scale_mat = cf::scaling_matrix(
        2. * (right - left) / canvas_width,
        2. * (top - bottom) / canvas_height,
        0.,
    );

    let transform_mat = cf::mult_matrix_4(scale_mat, translation_mat);
    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat);

    gl.draw_arrays(GL::TRIANGLES, 0, (self.rect_vertice_array_length / 2) as i32);
  }
}