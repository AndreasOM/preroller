
use glium::{glutin, Surface};

use glutin::ContextBuilder;

use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;


pub struct PreRollerBuilder {
	windowed: bool,
}

impl PreRollerBuilder {
	pub fn new() -> Self {
		Self {
			windowed: false,
		}
	}

	pub fn build( &self ) -> PreRoller {
		PreRoller {
			windowed: self.windowed,
		}
	}

	pub fn set_windowed( mut self, windowed: bool ) -> PreRollerBuilder {
		self.windowed = windowed;
		self
	}
}

#[derive(Debug, PartialEq)]
pub struct PreRoller {
	windowed: bool,
}

impl PreRoller {
	pub async fn run( &mut self ) -> anyhow::Result<()> {
		println!("PreRoller::run()");
		let el = EventLoop::new();
    	let wb = WindowBuilder::new()
        			.with_title("A transparent window!")
        			.with_decorations(false)
        			.with_transparent(true)
        			;
		let cb = glutin::ContextBuilder::new();

		// :TODO: handle actual fullscreen
		
    	let display = glium::Display::new(wb, cb, &el).unwrap();

	    el.run(move |event, _, control_flow| {
	        let next_frame_time = std::time::Instant::now() +
	            std::time::Duration::from_nanos(16_666_667);
	        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

	        match event {
	            glutin::event::Event::WindowEvent { event, .. } => match event {
	                glutin::event::WindowEvent::CloseRequested => {
	                    *control_flow = glutin::event_loop::ControlFlow::Exit;
	                    return;
	                },
		            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
		            	match ( input.virtual_keycode, input.state ) {
	                		( Some( VirtualKeyCode::Escape ), ElementState::Released ) => {
	                			*control_flow = ControlFlow::Exit;
	                			()
	                		} ,
	                		( Some( VirtualKeyCode::Escape ), ElementState::Pressed ) => {
	                			()
	                		} ,
	                		_ => {
	                			println!("Unhandled KeyboardInput {:?} {:?}", input.virtual_keycode, input.state );
	                			()
	                		},

		            	}

		            },
	                _ => return,
	            },
	            glutin::event::Event::NewEvents(cause) => match cause {
	                glutin::event::StartCause::ResumeTimeReached { .. } => (),
	                glutin::event::StartCause::Init => (),
	                _ => return,
	            },
	            _ => return,
	        }

	        let mut target = display.draw();
	        target.clear_color(0.0, 0.0, 1.0, 1.0);
	        target.finish().unwrap();
	    });
	}
}
