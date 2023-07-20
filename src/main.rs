use glium::{glutin::*, Surface, implement_vertex};


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let events_loop: event_loop::EventLoop<()> = event_loop::EventLoop::new();
    let wb = window::WindowBuilder::new()
        .with_inner_size(dpi::LogicalSize::new(1000, 600))
        .with_title("Platformer");
    let cb = ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = include_str!("shaders/triangle.vert");
    let fragment_shader_src = include_str!("shaders/triangle.frag");
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut frame = display.draw();
    frame.clear_color(0.3, 0.3, 0.5, 1.0);
    frame.finish().unwrap();


    loop {
        events_loop.run(move |event, _, control_flow| {
            match event {
                event::Event::WindowEvent { event, .. } => match event {
                    event::WindowEvent::CloseRequested => {
                        *control_flow = event_loop::ControlFlow::Exit;
                    }
                    event::WindowEvent::Resized(new_size) => {
                        display.gl_window().resize(new_size);
                    }
                    _ => {}
                },
                _ => {}
            }

            let mut frame = display.draw();
            frame.clear_color(0.3, 0.3, 0.5, 1.0);
            frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                &Default::default()).unwrap();
            frame.finish().unwrap();
        });
    }
}
