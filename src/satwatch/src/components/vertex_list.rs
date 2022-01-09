use glam::f32::*;
use glow::{Buffer, Context, HasContext, VertexArray};

use crate::util::evil_unsafe::u8_slice_from_any;

#[derive(Debug)]
pub struct VertexList {
    draw_type: u32,
    vao: Option<VertexArray>,
    _vbo: Option<Buffer>,
    _indices: Option<Buffer>,
    _normals: Option<Buffer>,
    element_count: i32,
}

impl VertexList {
    pub fn create(
        gl: &Context,
        draw_type: u32,
        vertices: &[Vec3],
        vertex_index: Option<&[u32]>,
        vertex_normals: Option<&[Vec3]>,
    ) -> Result<Self, String> {
        let vao = Some(unsafe { gl.create_vertex_array() }?);
        let vbo = Some(unsafe { gl.create_buffer() }?);
        unsafe {
            gl.bind_vertex_array(vao);
            gl.bind_buffer(glow::ARRAY_BUFFER, vbo);
            u8_slice_from_any(vertices, |slice| {
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, slice, glow::STATIC_DRAW);
            });
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                (core::mem::size_of::<f32>() * 3) as i32,
                0,
            );
            gl.enable_vertex_attrib_array(0);
        }

        // if present, index buffer it
        let indices = if let Some(idx) = vertex_index {
            unsafe {
                let bo = gl.create_buffer()?;
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(bo));
                u8_slice_from_any(idx, |slice| {
                    gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, slice, glow::STATIC_DRAW);
                    gl.vertex_attrib_pointer_f32(
                        1,
                        1,
                        glow::UNSIGNED_INT,
                        false,
                        (core::mem::size_of::<u32>()) as i32,
                        0,
                    );
                    gl.enable_vertex_attrib_array(1);
                });
                // urgh
                Some(bo)
            }
        } else {
            None
        };

        // if present, normal around
        let normals = if let Some(normals) = vertex_normals {
            unsafe {
                let normal_buffer = gl.create_buffer()?;
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(normal_buffer));
                u8_slice_from_any(normals, |slice| {
                    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, slice, glow::STATIC_DRAW);
                    gl.vertex_attrib_pointer_f32(
                        2,
                        3,
                        glow::FLOAT,
                        false,
                        (core::mem::size_of::<f32>() * 3) as i32,
                        0,
                    );
                    gl.enable_vertex_attrib_array(2);
                });

                Some(normal_buffer)
            }
        } else {
            None
        };

        let element_count = if indices.is_some() {
            vertex_index.unwrap().len()
        } else {
            vertices.len()
        } as i32;

        Ok(Self {
            draw_type,
            vao,
            _vbo: vbo,
            _indices: indices,
            element_count,
            _normals: normals,
        })
    }

    pub fn create_triangles(
        gl: &Context,
        vertices: &[Vec3],
        indices: Option<&[u32]>,
        vertex_normals: Option<&[Vec3]>,
    ) -> Result<Self, String> {
        Self::create(gl, glow::TRIANGLES, vertices, indices, vertex_normals)
    }

    pub fn create_lines(
        gl: &Context,
        vertices: &[Vec3],
        indices: Option<&[u32]>,
        vertex_normals: Option<&[Vec3]>,
    ) -> Result<Self, String> {
        Self::create(gl, glow::LINES, vertices, indices, vertex_normals)
    }

    pub fn draws_indexed(&self) -> bool {
        self._indices.is_some()
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.bind_vertex_array(self.vao);
        }
    }

    pub fn bind_and_draw(&self, gl: &Context) {
        self.bind(gl);
        if self.draws_indexed() {
            unsafe {
                gl.draw_elements(self.draw_type, self.element_count, glow::UNSIGNED_INT, 0);
            }
        } else {
            unsafe {
                gl.draw_arrays(self.draw_type, 0, self.element_count);
            }
        }
    }
}
