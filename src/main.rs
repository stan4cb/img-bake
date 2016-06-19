extern crate image;
extern crate qrcode;
extern crate time;

// https://github.com/kennytm/qrcode-rust
// http://www.piston.rs/image/image/

//use std::fs::File;
//use std::path::Path;

use qrcode::QrCode;
use image::GrayImage;

//use image::GenericImage;

fn main() {
    let starting_time = time::precise_time_s();


    /*
    let img = image::open(&Path::new("test.jpg")).unwrap();
    println!("dimensions {:?}", img.dimensions());

    let mut out = img.rotate270();

    out = out.resize(500,500, image::FilterType::Lanczos3);

    let ref mut fout = File::create(&Path::new("out.jpg")).unwrap();

    let _ = out.save(fout, image::JPEG).unwrap();
*/
    qr_generate("qr.png",b"http://www.selfmetric.com/wp-content/uploads/2016/06/TR45497890654_180616_1857.jpg");

    println!("Running for {:?}",time::precise_time_s() - starting_time);
}

fn qr_generate(file_name : &str,url : &[u8])
{
    match QrCode::new(url) {
        Err( error ) => println!("{:?}", error),
        Ok( result ) =>
        {
            let img : GrayImage = result.render().to_image();

            println!("QR dimensions {:?}", img.dimensions());

            img.save(file_name).unwrap();
        },
    }

}
