extern crate image;

use std::fs::File;
use std::path::Path;

use image::GenericImage;

fn main() {
    let img = image::open(&Path::new("test.jpg")).unwrap();
    println!("dimensions {:?}", img.dimensions());

    let out = img.rotate270();

    let ref mut fout = File::create(&Path::new("out.jpg")).unwrap();

    let _ = out.save(fout, image::JPEG).unwrap();
}
