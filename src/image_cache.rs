
use std::collections::VecDeque;
use std::path::PathBuf;

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

		let mut pending = VecDeque::new();

		for e in glob::glob( &g ).expect("Failed to read glob pattern") {
			match e {
				Ok( p ) => {
					while pending.len() > 2000 {
						let out: tokio::task::JoinHandle< _ > = pending.pop_front().unwrap();
					    self.images.push( out.await.unwrap() );
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
				},
				x => todo!("{:?}", x ),
			}
			/*
			*/
    	}

		while pending.len() > 0 {
			let out: tokio::task::JoinHandle< _ > = pending.pop_front().unwrap();
		    self.images.push( out.await.unwrap() );
		};

	}

}
