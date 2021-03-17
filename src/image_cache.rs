
//use image::DynamicImage::ImageRgba8;
use image::Rgba;
use glium::texture::RawImage2d;

//#[derive(Debug)]
pub struct ImageCache {
	images: Vec< Option< image::DynamicImage> >,
//	images: Vec< Option< RawImage2d<'a, u8> > >,
}

impl ImageCache {
	pub fn new() -> Self {
		Self {
			images: Vec::new(),
		}
	}

	pub fn get_image( &self, index: usize ) -> &Option< image::DynamicImage > {
		self.images.get( index ).unwrap_or( &None )
//		None
	}
/*
	fn load_image( maybe_image: &mut Option< image::ImageBuffer<Rgba<u8>, Vec<u8>> >, filename: &str ) {
		use std::io::Cursor;
		let image = image::load(Cursor::new(&include_bytes!("../data/loop/0120.png")[..]),
		                        image::ImageFormat::Png).unwrap().to_rgba8();
		*maybe_image = Some( image );
	}
*/
	fn load_image( &mut self, index: usize, filename: &str ) {
		use std::io::Cursor;
		let image = image::load(Cursor::new(&include_bytes!("../data/loop/0120.png")[..]),
		                        image::ImageFormat::Png).unwrap().to_rgba8();
		let image_dimensions = image.dimensions();
		self.images.push( Some( image::DynamicImage::ImageRgba8( image ) ) );
//		self.images.push( Some( image ) );
		/*
		let maybe_image = self.images.get_mut( index );
		maybe_image = Some( image );
		*/
	}

	pub async fn load_images( &mut self, path: &str ) {
		/*
					self.images_loop = Vec::with_capacity( 2 );
			for _ in 0..2 {
				self.images_loop.push( None );
			}

			let (left, right) = self.images_loop.split_at_mut(1);
			Self::load_image( &mut left[ 0 ], "data/loop/0120.png");
			Self::load_image( &mut right[ 0 ], "data/loop/0120.png");
		*/
		self.load_image( 0, "data/loop/0120.png" );
	}

}
