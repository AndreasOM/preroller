
use clap::{App, Arg};

use preroller::preroller::{ PreRoller, PreRollerBuilder };

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

	println!("Prerolling {}, {}, {} at {} fps {}", in_path, loop_path, out_path, fps, if windowed { "[windowed]" } else { "" } );

	let builder = PreRollerBuilder::new()
					.set_windowed(windowed);

	let mut preroller = builder.build();

	preroller.run().await?;

	Ok(())
}
