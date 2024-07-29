use std::num::NonZeroU32;

use event::AppEvent;
use winit::platform::wayland::WindowAttributesExtWayland;

use crate::{widget::Widget, Canvas};

pub(crate) mod app_context;
pub(crate) mod assets;
pub(crate) mod event;

enum GraphicState {
    Created {
        window: std::rc::Rc<winit::window::Window>,
        _context: softbuffer::Context<std::rc::Rc<winit::window::Window>>,
        surface: softbuffer::Surface<std::rc::Rc<winit::window::Window>, std::rc::Rc<winit::window::Window>>,
    },
    ToBeCreated {
        window_attributes: winit::window::WindowAttributes
    },
    Failed {
        error: winit::error::OsError,
    }
}

pub struct ApplicationData<UserEvent, Root: Widget<UserEvent>> {
    _m: core::marker::PhantomData<UserEvent>,
    graphic_state: GraphicState,
    root: Root,
    assets: assets::Assets,
}

impl<UserEvent: 'static, Root: Widget<UserEvent>> winit::application::ApplicationHandler<UserEvent> for ApplicationData<UserEvent, Root> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match &self.graphic_state {
            GraphicState::ToBeCreated { window_attributes } => match event_loop.create_window(window_attributes.clone()) {
                Ok(window) => {

                    let window = std::rc::Rc::new(window);
                    let context = softbuffer::Context::new(window.clone()).unwrap();
                    let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

                    self.graphic_state = GraphicState::Created { 
                        window,
                        _context: context,
                        surface
                    }
                }
                Err(error) => {
                    println!("Failed to create window: {error}");
                    self.graphic_state = GraphicState::Failed { error };
                },
            }
            GraphicState::Created { .. } => {
                // Maybe we have to recreate graphic events here ?
                println!("received resume event with graphic state created!")
            }
            GraphicState::Failed { error } => {
                // what to handle here ?
                println!("received resume event with a failed graphic state (error: {error})!")
            },
        }
    }

    fn window_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _window_id: winit::window::WindowId, event: winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::Resized(size) => match &mut self.graphic_state {
                GraphicState::Created { surface, window, .. } => {
                    match (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                        (Some(width), Some(height)) => {
                            let _ = surface.resize(width, height); // todo handle error
                            window.request_redraw()
                        },
                        _ => {}, // window got resized to size 0, ignore. It wont be drawn anyway.
                    }
                },
                _ => {}, // window does not exist
            },
            winit::event::WindowEvent::RedrawRequested => self.redraw(),
            other_event => match (&self.graphic_state, AppEvent::<UserEvent>::try_from(other_event)) {
                (GraphicState::Created { window, .. }, Some(event)) => {
                    let inner_size = window.inner_size();
                    let rect = crate::Rect { x: 0, y: 0, width: NonZeroU32::new(inner_size.width).unwrap(), height: NonZeroU32::new(inner_size.height).unwrap() };
                    let _ = self.root.handle_event(&event, rect);

                },
                _ => {}
            }
        }
    }
}

impl<UserEvent, Root: Widget<UserEvent>> ApplicationData<UserEvent, Root> {
    pub fn redraw(&mut self) {
        match &mut self.graphic_state {
            GraphicState::Created { window, surface, .. } => {
                let inner_size = window.inner_size();
                if (inner_size.width & inner_size.height) > 0 {
                    let rect = crate::Rect {
                        x: 0, y: 0, width: NonZeroU32::new(inner_size.width).unwrap(), height: NonZeroU32::new(inner_size.height).unwrap(),
                    };
                
                    let mut canvas = Canvas::new(
                        &mut self.assets,
                        surface,
                        inner_size.width,
                        inner_size.height,
                    );
                
                    self.root.draw(&mut canvas, rect);
                    canvas.present().unwrap();
                }
            },
            _ => {},
        }
    }
}


pub fn run_app<UserEvent: 'static, Root: Widget<UserEvent>>(root: Root, window_attrs: Option<winit::window::WindowAttributes>) -> Result<(), winit::error::EventLoopError> {
    
    let assets = assets::Assets::new();
    let event_loop = winit::event_loop::EventLoop::with_user_event().build().unwrap();

    let window_attributes = match window_attrs {
        Some(attrs) => attrs,
        None => winit::window::WindowAttributes::default()
            .with_name("Nerf", "Nerf"),
    };
    
    let mut app = ApplicationData {
        _m: core::marker::PhantomData,
        graphic_state: GraphicState::ToBeCreated { window_attributes },
        root,
        assets,
    };

    event_loop.run_app(&mut app)
}
