
use std::time::{Duration, Instant};

use glium::{glutin, Surface};

use glutin::ContextBuilder;

use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;

use image::Rgba;
use image::GenericImageView;


use crate::fullscreen_quad::FulllscreenQuad;
use crate::image_cache::ImageCache;

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
			state: State::InWait,
			current_image: 0,
//			images_in: ImageCache::new(),
//			images_loop: ImageCache::new(),
//			images_out: ImageCache::new(),
		}
	}

	pub fn set_windowed( mut self, windowed: bool ) -> PreRollerBuilder {
		self.windowed = windowed;
		self
	}
}

#[derive(Debug, PartialEq)]
enum State {
	InWait,		// wait to start
	In,
	Loop,
	LoopFinish,	// finish loop once more
	Out,
	OutDone		// wait to close
}

#[derive(Debug)]
pub struct PreRoller {
	windowed: bool,
	state: State,
	current_image: usize,
//	images_in: ImageCache,
//	images_loop: ImageCache,
//	images_out: ImageCache,
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

		let fsq = FulllscreenQuad::new( &display );


		// load all images
		// :HACK:
		{
//			self.images_loop.load_images( "loop" );
		}

		let start = Instant::now();
		let mut images_in = ImageCache::new();
		images_in.load_images( "data/in/*.png" ).await;

		let mut images_loop = ImageCache::new();
		images_loop.load_images( "data/loop/*.png" ).await;

		let mut images_out = ImageCache::new();
		images_out.load_images( "data/out/*.png" ).await;

	    let duration = start.elapsed();

	    println!("Loaded in {:?}", duration);

		let mut state = State::InWait;
		let mut current_image = 0;


		println!("Waiting for SPACE...");

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
	                			println!("Bye bye...");
	                			*control_flow = ControlFlow::Exit;
	                			()
	                		} ,
	                		( Some( VirtualKeyCode::Escape ), ElementState::Pressed ) => {
	                			()
	                		} ,
	                		( Some( VirtualKeyCode::Space ), ElementState::Released ) => {
	                			println!("SPACE released");
						        match &state {
									State::InWait => {
										println!("Going in!");
										state = State::In;
										current_image = 0;
									},
									State::In => {
									},
									State::Loop => {
										println!("Finishing loop!");
										state = State::LoopFinish;
									},
									State::LoopFinish => {
									}
									State::Out => {
									},
									State::OutDone => {
									},
						        }
	                			()
	                		} ,
	                		( Some( VirtualKeyCode::Space ), ElementState::Pressed ) => {
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
// load texture every frame
			{
				/*
				use std::io::Cursor;
				let image = image::load(Cursor::new(&include_bytes!("../data/loop/0120.png")[..]),
				                        image::ImageFormat::Png).unwrap().to_rgba8();
				*/
//				dbg!(&state, current_image);
				let image = match state {
					State::InWait | State::In => {
						images_in.get_image( current_image )
					},
					State::Loop | State::LoopFinish => {
						images_loop.get_image( current_image )
					},
					State::Out | State::OutDone => {
						images_out.get_image( current_image )
					},
					_ => &None,
				};
				match image { // images_loop.get_image( current_image ) /*self.get_image()*/ /*Some( image )*/ {
					Some( image ) => {
				        let mut target = display.draw();
				        target.clear_color(0.0, 0.0, 1.0, 1.0);

						let image_dimensions = image.dimensions();
//						let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.to_rgba8().into_raw(), image_dimensions);
						let image = glium::texture::RawImage2d::from_raw_rgba(image.to_rgba8().into_raw(), image_dimensions);
//						let image = &*image.clone();
						let texture = glium::texture::Texture2d::new(&display, image).unwrap();

				        fsq.render( &mut target, &texture );
				        target.finish().unwrap();
				        drop(texture);


				        match &state {
							State::InWait => {
								// advance on key
							},
							State::In => {
								current_image += 1;
							},
							State::Loop => {
								current_image += 1;
							},
							State::LoopFinish => {
								current_image += 1;
							}
							State::Out => {
								current_image += 1;
							},
							State::OutDone => {
								*control_flow = ControlFlow::Exit;
							},
				        }
					},
					None => {
				        match &state {
							State::InWait => {
								// advance on key
							},
							State::In => {
								state = State::Loop;
								current_image = 0;
								println!("Looping!");
							},
							State::Loop => {
								current_image = 0;
								println!("Looping again...");
							},
							State::LoopFinish => {
								state = State::Out;
								current_image = 0;
								println!("Going out!");
							}
							State::Out => {
								state = State::OutDone;
								println!("Done!");
							},
							State::OutDone => {
								*control_flow = ControlFlow::Exit;								
							},
				        }
					},
				}

//		        println!("Frame done");
	    	}

	    });
	}
}
