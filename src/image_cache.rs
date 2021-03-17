
use std::collections::VecDeque;
use std::path::PathBuf;

use indicatif::{ ProgressBar, ProgressStyle };

//use image::DynamicImage::ImageRgba8;

//#[derive(Debug)]
pub struct ImageCache {
	images: Vec< Option< image::DynamicImage> >,
}

impl ImageCache {
	pub fn new() -> Self {
		Self {
			images: Vec::new(),
		}
	}

	pub fn len( &self ) -> usize {
		self.images.len()
	}

	pub fn get_image( &self, index: usize ) -> &Option< image::DynamicImage > {
		self.images.get( index ).unwrap_or( &None )
	}

	async fn load_image( filename: PathBuf ) -> Option< image::DynamicImage > {
//		println!("Loading {:?}", &filename);
		let image = image::open(filename).unwrap().to_rgba8();
		Some( image::DynamicImage::ImageRgba8( image ) )		
	}
	/*
	async fn load_image( &mut self, index: usize, filename: &PathBuf ) {
		let image = image::open(filename).unwrap().to_rgba8();
		self.images.push( Some( image::DynamicImage::ImageRgba8( image ) ) );
	}
	*/
	pub async fn load_images( &mut self, path: &str ) {
		let dir = std::env::current_dir().unwrap();
		let g = format!("{}/{}", dir.as_path().to_string_lossy(), path.to_string());
		dbg!(&g);

		let mut to_load = VecDeque::new();
		for e in glob::glob( &g ).expect("Failed to read glob pattern") {
			match e {
				Ok( p ) => {
					to_load.push_back( p );
				},
				_ => {},
			}
		};

    	let pb = ProgressBar::new( to_load.len() as u64 );
		pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {eta_precise} {msg}")
                .progress_chars("█▓▒░  "),
        );

		let mut pending = VecDeque::new();
//		for e in glob::glob( &g ).expect("Failed to read glob pattern") {
		for p in to_load {
//			match e {
//				Ok( p ) => {
					while pending.len() > 2000 {
						let out: tokio::task::JoinHandle< _ > = pending.pop_front().unwrap();
					    self.images.push( out.await.unwrap() );
					    pb.inc(1);
					};
//					println!("{}", &p.display() );
					let out = tokio::spawn(async move {
						ImageCache::load_image( p ).await
				    });
//					dbg!(&out);
					pending.push_back( out );
//					pending.push( handle );
//				    let out = handle.await.unwrap();
//				    self.images.push( out.await.unwrap() );
//				},
//				x => todo!("{:?}", x ),
//			}
			/*
			*/
    	}

		while pending.len() > 0 {
			let out: tokio::task::JoinHandle< _ > = pending.pop_front().unwrap();
		    self.images.push( out.await.unwrap() );
		    pb.inc(1);
		};

		pb.finish_with_message("done");
	}

}
