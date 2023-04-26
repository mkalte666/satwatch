use crate::components::DirectionalLight;
use crate::util::asset_file::{asset_file_name, asset_file_str};
use glam::f32::*;
use glow::{Context, HasContext, Program, Shader, Texture, UniformLocation};
use sdl2::pixels::PixelFormatEnum;
use serde::Deserialize;

#[derive(Debug)]
pub struct Material {
    program: Program,
    draw_color: Option<Vec4>,
    draw_color_location: Option<UniformLocation>,
    mvp_location: Option<UniformLocation>,
    dirlight_dir_location: Option<UniformLocation>,
    dirlight_color_location: Option<UniformLocation>,
    dirlight_ambient_location: Option<UniformLocation>,
    cubemap: Option<glow::Texture>,
    cubemap_location: Option<UniformLocation>,
}

#[derive(Deserialize, Debug)]
struct MaterialFile {
    #[allow(dead_code)]
    name: String,
    shaders: MaterialFileShaders,
    parameters: Option<MaterialFileParameters>,
}

#[derive(Deserialize, Debug)]
struct MaterialFileShaders {
    vertex: String,
    fragment: String,
}

#[derive(Deserialize, Debug)]
struct MaterialFileParameters {
    draw_color: Option<Vec4>,
    cubemap_faces: Option<Vec<String>>,
}

fn compile_shader(gl: &Context, shader_type: u32, source: &str) -> Result<Shader, String> {
    unsafe {
        let shader = gl.create_shader(shader_type)?;
        gl.shader_source(shader, source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            return Err(format!(
                "Shader Compilation failed: {}",
                gl.get_shader_info_log(shader)
            )
            .to_string());
        }

        return Ok(shader);
    }
}

fn link_program(gl: &Context, shaders: &[Shader]) -> Result<Program, String> {
    unsafe {
        let program = gl.create_program()?;
        for shader in shaders {
            gl.attach_shader(program, *shader);
        }
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            return Err(format!(
                "Program Linking failed: {}",
                gl.get_program_info_log(program)
            )
            .to_string());
        }

        return Ok(program);
    }
}

fn make_cubemap(gl: &glow::Context, files: &[String]) -> Result<Texture, String> {
    if files.len() != 6 {
        return Err("Cubemap needs exactly 6 faces".to_string());
    }

    use sdl2::image::LoadSurface;

    unsafe {
        let tex = gl.create_texture()?;
        for i in 0..6 {
            gl.bind_texture(glow::TEXTURE_CUBE_MAP, Some(tex));
            let surface_unformatted: sdl2::surface::Surface =
                LoadSurface::from_file(asset_file_name(files.get(i).unwrap())?)?;
            let surface = surface_unformatted.convert_format(PixelFormatEnum::ABGR8888)?;
            gl.tex_image_2d(
                glow::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                0,
                glow::RGBA8 as i32,
                surface.width() as i32,
                surface.height() as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                surface.without_lock(),
            );
        }

        gl.tex_parameter_i32(
            glow::TEXTURE_CUBE_MAP,
            glow::TEXTURE_MIN_FILTER,
            glow::LINEAR as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_CUBE_MAP,
            glow::TEXTURE_MAG_FILTER,
            glow::LINEAR as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_CUBE_MAP,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_CUBE_MAP,
            glow::TEXTURE_WRAP_R,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_CUBE_MAP,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );

        Ok(tex)
    }
}

impl Material {
    pub fn from_file(gl: &Context, filename: &str) -> Result<Self, String> {
        // lets read this
        let file_str = asset_file_str(filename)?;
        let file_res = toml::from_str(&file_str);
        if file_res.is_err() {
            return Err(format!(
                "Error in material file {}: {}",
                filename,
                file_res.err().unwrap().to_string()
            ));
        }
        let file: MaterialFile = file_res.ok().unwrap();
        let vertex_source = asset_file_str(&file.shaders.vertex)?;
        let fragment_source = asset_file_str(&file.shaders.fragment)?;

        // shaders first
        let mut shaders = Vec::new();
        shaders.push(compile_shader(gl, glow::VERTEX_SHADER, &vertex_source)?);
        shaders.push(compile_shader(gl, glow::FRAGMENT_SHADER, &fragment_source)?);
        let program = link_program(gl, &shaders)?;

        let draw_color = if file.parameters.is_some()
            && file.parameters.as_ref().unwrap().draw_color.is_some()
        {
            Some(file.parameters.as_ref().unwrap().draw_color.unwrap())
        } else {
            None
        };

        let draw_color_location = if draw_color.is_some() {
            unsafe { gl.get_uniform_location(program, "draw_color") }
        } else {
            None
        };

        // fixed things needed elsewehre that live in shaders
        let mvp_location = unsafe { gl.get_uniform_location(program, "mvp") };
        let dirlight_dir_location = unsafe { gl.get_uniform_location(program, "dirlight_dir") };
        let dirlight_color_location = unsafe { gl.get_uniform_location(program, "dirlight_color") };
        let dirlight_ambient_location =
            unsafe { gl.get_uniform_location(program, "dirlight_ambient") };

        let cubemap_location = unsafe { gl.get_uniform_location(program, "cubemap") };

        let mut cubemap = None;
        if let Some(parameters) = file.parameters.as_ref() {
            if let Some(filenames) = parameters.cubemap_faces.as_ref() {
                cubemap = Some(make_cubemap(gl, &filenames)?);
            }
        }

        Ok(Self {
            program,
            draw_color,
            draw_color_location,
            mvp_location,
            dirlight_dir_location,
            dirlight_color_location,
            dirlight_ambient_location,
            cubemap_location,
            cubemap,
        })
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.use_program(Some(self.program));
            if self.draw_color_location.is_some() {
                let slice = &self.draw_color.unwrap();
                gl.uniform_4_f32(
                    self.draw_color_location.as_ref(),
                    slice.x,
                    slice.y,
                    slice.z,
                    slice.w,
                );
            }
            if let Some(location) = &self.cubemap_location {
                if let Some(cubemap) = self.cubemap {
                    gl.uniform_1_u32(Some(location), cubemap.0.into());
                    gl.bind_texture(glow::TEXTURE_CUBE_MAP, Some(cubemap));
                }
            }
        }
    }

    pub fn set_directional_light(&self, gl: &Context, dirlight: DirectionalLight) {
        unsafe {
            if self.dirlight_dir_location.is_some() {
                gl.uniform_3_f32(
                    self.dirlight_dir_location.as_ref(),
                    dirlight.direction.x,
                    dirlight.direction.y,
                    dirlight.direction.z,
                );
            }
            if self.dirlight_color_location.is_some() {
                gl.uniform_4_f32(
                    self.dirlight_color_location.as_ref(),
                    dirlight.color.x,
                    dirlight.color.y,
                    dirlight.color.z,
                    dirlight.color.w,
                );
            }
            if self.dirlight_ambient_location.is_some() {
                gl.uniform_1_f32(self.dirlight_ambient_location.as_ref(), dirlight.ambient);
            }
        }
    }

    pub fn set_mvp(&self, gl: &Context, mvp: Mat4) {
        unsafe {
            let slice: &[f32; 16] = mvp.as_ref();
            gl.uniform_matrix_4_f32_slice(self.mvp_location.as_ref(), false, slice.as_slice());
        }
    }
}
