use glium::{glutin::*, Surface};

fn main() {
    let events_loop: event_loop::EventLoop<()> = event_loop::EventLoop::new();
    let wb = window::WindowBuilder::new()
        .with_inner_size(dpi::LogicalSize::new(1000, 600))
        .with_title("Platformer");
    let cb = ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

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
            frame.finish().unwrap();
        });
    }
}
