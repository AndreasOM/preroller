
use glium::*;//uniform;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub struct FulllscreenQuad {
	vertex_buffer: VertexBuffer< Vertex >,
	indices: glium::index::NoIndices,
    program: Program,
}

impl FulllscreenQuad {

	pub fn new( display: &Display ) -> Self {

	    let vertex1 = Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 0.0] };
	    let vertex2 = Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 1.0] };
	    let vertex3 = Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 0.0] };
	    let vertex4 = Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 1.0] };
	    let shape = vec![vertex1, vertex2, vertex3, vertex2, vertex4, vertex3 ];

	    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
	    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

		let vertex_shader_src = r#"
	        #version 140
	        in vec2 position;
	        in vec2 tex_coords;
	        out vec2 v_tex_coords;
	        uniform mat4 matrix;
	        void main() {
	            v_tex_coords = tex_coords;
//	            gl_Position = matrix * vec4(position, 0.0, 1.0);
	            gl_Position = vec4(position, 0.0, 1.0);
	        }
	    "#;

	    let fragment_shader_src = r#"
	        #version 140
	        in vec2 v_tex_coords;
	        out vec4 color;
	        uniform sampler2D tex;
	        void main() {
	            color = texture(tex, v_tex_coords);
	        }
	    "#;

	    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

		Self {
			vertex_buffer,
			indices,
			program,
		}
	}

	pub fn render( &self, target: &mut glium::Frame, texture: &glium::Texture2d ) {
		let uniforms = uniform! {
            tex: texture,
        };

        target.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms,
                    &Default::default()).unwrap();
	}
}
