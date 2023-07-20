use std::num::NonZeroU32;

use crate::{widget::Widget, drawing::canvas::Canvas};

use self::event::{nerf_event::NerfEvent, input_event::InputEvent};

pub(crate) mod event;
pub(crate) mod app_context;

pub struct App {
    window: winit::window::Window,
    event_loop: Option<winit::event_loop::EventLoop<NerfEvent>>,
    #[allow(unused)]
    context: softbuffer::Context,
    surface: softbuffer::Surface,
    root: Box<dyn Widget>,
    request_redraw: bool,
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
            event_loop: Some(event_loop),
            context,
            surface,
            root,
            request_redraw: true,
        }
    }

    pub fn run(mut self) {
        let event_loop = self.event_loop.take().unwrap();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Wait;
            match event {
                winit::event::Event::WindowEvent { event, .. } => self.handle_window_event(
                    control_flow,
                    event,
                ),
                winit::event::Event::MainEventsCleared => if self.request_redraw {
                    self.request_redraw = false;
                    self.window.request_redraw();
                }
                winit::event::Event::RedrawRequested(window_id) => if window_id == self.window.id() {
                    let (width, height) = {
                        let size = self.window.inner_size();
                        (size.width, size.height)
                    };
                    let rect = match (NonZeroU32::new(width), NonZeroU32::new(height)) {
                        (Some(width), Some(height)) => softbuffer::Rect {
                            x: 0, y: 0, width, height,
                        },
                        _ => return, // unable to draw to size 0 canvas (+ useless)
                    };
                    let mut canvas = Canvas::new(&mut self.surface, width, height);

                    self.root.draw(&mut canvas, rect);

                    let _ = canvas.buffer().present(); // todo handle error
                }
                _ => {}
            }
        });
    }

    fn handle_window_event(
        &mut self,
        control_flow: &mut winit::event_loop::ControlFlow,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            winit::event::WindowEvent::Resized(size) => match (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                (Some(width), Some(height)) => {
                    let _ = self.surface.resize(width, height); // todo handle error
                    self.request_redraw = true;
                },
                _ => {}, // window got resized to size 0, ignore. It wont be drawn anyway.
            },
            _ => match InputEvent::try_from(event) {
                Some(event) => {
                    let (width, height) = {
                        let size = self.window.inner_size();
                        (size.width, size.height)
                    };
                    let rect = match (NonZeroU32::new(width), NonZeroU32::new(height)) {
                        (Some(width), Some(height)) => softbuffer::Rect {
                            x: 0, y: 0, width, height,
                        },
                        _ => return, // unable to handle event for size 0 rect, as we need to pass a rect. should we handle anyway ?
                    };
                    let response = self.root.handle_event(event, rect);
                    self.handle_response_flags(response);
                },
                None => {}
            }
        }
    }

    fn handle_response_flags(&mut self, response: event::event_responses::EventResponse) {
        if response.contains(event::event_responses::EventResponse::REDRAW_REQUEST) {
            self.request_redraw = true;
        }
    }


}