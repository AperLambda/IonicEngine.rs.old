extern crate gl;
extern crate glfw;
extern crate ionic_engine;

use ionic_engine::system::context::{IonicContext, WindowMode};
use ionic_engine::graphics::*;
use ionic_engine::*;

fn main()
{
    let context = IonicContext::new();
    let mut glfw = context.get_glfw();

    let mut iwindow = context.create_window("Test Window", 600, 600, WindowMode::Windowed);
    println!("Window: Title: '{}', Width: {}, Height: {}", iwindow.get_title(), iwindow.get_size().0, iwindow.get_size().1);

    {
        let mut window = &mut iwindow.handle;

        window.set_key_polling(true);
    }

    iwindow.make_current();

    context.load_gl();

    let clear_color = COLOR_BLACK.to_float();

    let mut ig = IonicGraphics::new(0.0, 0.0, "src/shaders/vertex.glsl", "src/shaders/fragment.glsl");
    ig.bind_vbo(&VERTEX_TRI);
    ig.use_program();

    let color = ig.get_uniform_location("color");
    ig.set_uniform_float(color, 0.5);

    let posmodifier = ig.get_uniform_location("modifier");
    ig.set_uniform_2_float(posmodifier, 0.0, 0.1);

    let mut a: f32 = 0.0;
    while !iwindow.should_close()
        {
            a = a + 0.05;
            unsafe
                {
                    gl::ClearColor(clear_color.0, clear_color.1, clear_color.2, clear_color.3);
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    ig.use_program();
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                    gl::DrawArrays(gl::TRIANGLES, 3, 3);
                }
            ig.set_uniform_float(color, a.sin()*0.5+0.5);
            ig.set_uniform_2_float(posmodifier, a.sin()*0.2, a.cos()*0.2);
            iwindow.swap_buffers();
            glfw.poll_events();

            for (_, event) in glfw::flush_messages(&iwindow.events) {
                match event
                {
                    _ => {}
                }
            }
        }
}

static VERTEX_TRI: [gl::types::GLfloat; 12] = [
    0.0, 0.5,
    0.5, -0.5,
    -0.5, -0.5,

    0.0, 0.45,
    0.46, -0.475,
    -0.46, -0.475
];