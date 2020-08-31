use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::{GlProfile, GlRequest};

use gl;
use gl::types::*;

use std::ffi::CString;
use std::fs;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Boulder Dash")
        .with_inner_size(LogicalSize::new(1024.0, 768.0));
    let gl_request = GlRequest::Latest;
    let gl_profile = GlProfile::Core;
    let windowed_context = ContextBuilder::new()
        .with_gl(gl_request)
        .with_gl_profile(gl_profile)
        .with_double_buffer(Some(true))
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

    let window_size = windowed_context.window().inner_size();
    unsafe {
        gl::Viewport(0, 0, window_size.width as i32, window_size.height as i32);
        gl::ClearColor(0.95, 0.05, 0.05, 1.0);
    }

    // Triangle
    let vertices: Vec<f32> = vec![
        0.0, 0.5, 0.0, 1.0, 0.0, 0.0, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
    ];
    let mut buffer_id: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut buffer_id);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * 4) as isize,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
    }

    // Vertex shader
    let source = fs::read_to_string("shaders/shader.vert").unwrap();
    let source = CString::new(source).unwrap();
    let vert_shader_id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe {
        gl::ShaderSource(vert_shader_id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(vert_shader_id);
    }

    // Fragment shader
    let source = fs::read_to_string("shaders/shader.frag").unwrap();
    let source = CString::new(source).unwrap();
    let frag_shader_id = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
        gl::ShaderSource(frag_shader_id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(frag_shader_id);
    }

    // Shader program
    let program_id = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program_id, vert_shader_id);
        gl::AttachShader(program_id, frag_shader_id);
        gl::LinkProgram(program_id);
        gl::UseProgram(program_id);
    }

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    })
}
