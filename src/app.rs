use std::num::NonZeroU32;

use crate::{widget::Widget, drawing::canvas::Canvas};

use self::event::NerfEvent;

pub(crate) mod event;

pub struct App {
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<NerfEvent>,
    #[allow(unused)]
    context: softbuffer::Context,
    surface: softbuffer::Surface,
    root: Box<dyn Widget>,
}

impl App {
    pub fn new(root: Box<dyn Widget>) -> Self {

        let event_loop = winit::event_loop::EventLoopBuilder::with_user_event().build();
        let window = winit::window::WindowBuilder::new()
            .with_title("Nerf")
            .build(&event_loop)
            .unwrap();
        let context = unsafe { softbuffer::Context::new(&window).unwrap() };
        let mut surface = unsafe { softbuffer::Surface::new(&context, &window).unwrap() };
        surface.resize(
            NonZeroU32::new(window.inner_size().width).unwrap(),
            NonZeroU32::new(window.inner_size().height).unwrap(),
        ).unwrap();

        Self {
            window,
            event_loop,
            context,
            surface,
            root,
        }
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Wait;
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    winit::event::WindowEvent::Resized(size) => match (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                        (Some(width), Some(height)) => {
                            let _ = self.surface.resize(width, height); // todo handle error
                        },
                        _ => {}, // window got resized to size 0, ignore. It wont be drawn anyway.
                    }
                    _ => {}
                },
                winit::event::Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                winit::event::Event::RedrawRequested(window_id) => if window_id == self.window.id() {
                    let (width, height) = {
                        let size = self.window.inner_size();
                        (size.width, size.height)
                    };
                    let mut canvas = Canvas::new(&mut self.surface, width);
                    let rect = softbuffer::Rect {
                        x: 0,
                        y: 0,
                        width: match NonZeroU32::new(width) {
                            Some(size) => size,
                            None => return, // unable to draw to size 0 canvas (+ useless)
                        },
                        height: match NonZeroU32::new(height) {
                            Some(size) => size,
                            None => return, // unable to draw to size 0 canvas (+ useless)
                        },
                    };
                    self.root.draw(&mut canvas, rect);

                    let _ = canvas.buffer().present(); // todo handle error
                }
                _ => {}
            }
        });
    }
}