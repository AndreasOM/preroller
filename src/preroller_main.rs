
use clap::{App, Arg};

use preroller::preroller::PreRollerBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let matches = App::new("preroller")
							.version("0.1")
							.author("Andreas N. <andreas@omni-mad.com>")
							.arg( Arg::with_name("in")
								.long("in")
								.short("i")
								.value_name("IN")
								.help("Set the file/folder for the 'in' sequence.")
								.takes_value(true)
							)
							.arg( Arg::with_name("loop")
								.long("loop")
								.short("l")
								.value_name("LOOP")
								.help("Set the file/folder for the 'loop' sequence.")
								.takes_value(true)
							)
							.arg( Arg::with_name("out")
								.long("out")
								.short("o")
								.value_name("OUT")
								.help("Set the file/folder for the 'out' sequence.")
								.takes_value(true)
							)
							.arg( Arg::with_name("fps")
								.long("fps")
								.value_name("FPS")
								.help("Set the playback speed in fps.")
								.takes_value(true)
							)
							.arg( Arg::with_name("windowed")
								.long("windowed")
								.help("Run windowed (instead of fullscreen).")
								.takes_value(false)
							)
							.arg( Arg::with_name("threads")
								.long("threads")
								.value_name("THREADS")
								.help("Number of threads.")
								.takes_value(true)
							)
							.get_matches();

	let in_path = matches.value_of("in").unwrap_or("in").to_string();
	let loop_path = matches.value_of("loop").unwrap_or("loop").to_string();
	let out_path = matches.value_of("out").unwrap_or("out").to_string();

	let windowed = matches.occurrences_of("windowed") > 0;


	let fps = matches.value_of("fps").unwrap_or("25").to_string();
	let fps = match fps.parse::<f32>() {
		Ok( fps ) => fps,
		Err( _ ) => panic!("Invalid fps {:?}", fps ),
	};

	let threads = matches.value_of("threads").unwrap_or("1").to_string();
	let threads = match threads.parse::<usize>() {
		Ok( threads ) => threads,
		Err( _ ) => panic!("Invalid threads {:?}", threads ),
	};

	println!("Prerolling {}, {}, {} at {} fps {}", in_path, loop_path, out_path, fps, if windowed { "[windowed]" } else { "" } );

	let builder = PreRollerBuilder::new()
					.set_windowed(windowed);

	let mut preroller = builder.build();

	let runtime = tokio::runtime::Builder::new_multi_thread()
	        .worker_threads(threads)
	        .thread_name("preroller")
	        .thread_stack_size(3 * 1024 * 1024)
	        .build()
	        .unwrap();


	let _guard = runtime.enter();

	match preroller.run().await {
		Err( e ) => println!("Error: {:?}", e ),
		Ok( _ ) => {},
	}

	Ok(())
}
