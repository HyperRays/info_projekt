use pyo3::{prelude::*};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder,Window}
};

#[pyclass(unsendable)]
struct EventsHandler{
    event_loop: Option<EventLoop<()>>,
    window: Option<Window>
}

#[pymethods]
impl EventsHandler{
    #[new]
    fn new() -> Self{
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        Self { event_loop: Some(event_loop), window: Some(window)}
    }

    fn run(&mut self){
        let event_loop = self.event_loop.take().unwrap();
        let window = self.window.take().unwrap();
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        });
    }
}

#[pymodule]
fn graphics(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EventsHandler>()?;
    Ok(())
}

