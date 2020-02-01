extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::os::raw::c_void;
use std::ffi::{CString, CStr};
use std::ptr::null;
use std::ptr::null_mut;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}

struct Shader {
    id: gl::types::GLuint
}

impl Shader {
    fn from_source(source: &CStr, kind: gl::types::GLuint) -> Result<Shader, String> {
        let id = unsafe {
            gl::CreateShader(kind)
        };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;

        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let err = create_cstring(len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, len, null_mut(), err.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(err.to_string_lossy().into_owned());
        }

        Ok(Shader{ id })
    }

    fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

/*
fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe {
        gl::CreateShader(kind)
    };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;

    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let err = create_cstring(len as usize);

        unsafe {
            gl::GetShaderInfoLog(id, len, null_mut(), err.as_ptr() as *mut gl::types::GLchar);
        }

        return Err(err.to_string_lossy().into_owned());
    }

    Ok(id)
}
*/

fn create_cstring(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));

    unsafe { CString::from_vec_unchecked(buffer) }
}
