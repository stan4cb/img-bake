extern crate image; // procces
extern crate qrcode; // gen qr
extern crate time; // bench

// https://github.com/kennytm/qrcode-rust
// http://www.piston.rs/image/image/
// http://www.selfmetric.com/wp-content/uploads/2016/06/TR45497890654_180616_1857.jpg

use std::fs::File;
use std::path::Path;

use qrcode::QrCode;

use image::{ GrayImage , GenericImage , ImageBuffer ,  DynamicImage , Rgba , ConvertBuffer}; // imageops

fn main() {
    let starting_time = time::precise_time_s();

    let mut img = image::open(&Path::new("test.jpg")).unwrap();
    image4x(&mut img);

    update_time("Running", starting_time);
}

fn image4x(img : &mut DynamicImage) {
    let mut imgbuf : ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(1205, 1795);

    match qr_generate(b"test-url") {
        Err( err ) => println!("QR render error {:?}", err),
        Ok( qr_render ) =>
        {
            let mut qr_rgba : ImageBuffer<Rgba<u8>, Vec<u8>>= qr_render.convert();

            copy_to_buffer(&mut imgbuf, img, 0 , 0);
            copy_to_buffer(&mut imgbuf, img, 500 , 500);

            copy_b_to_buffer(&mut imgbuf, &mut qr_rgba , 0 , 1795 / 2);
            copy_b_to_buffer(&mut imgbuf, &mut qr_rgba , 1205 / 2 , 1795 / 2);

            let ref mut fout = File::create(&Path::new("example.jpg")).unwrap();
            let _ = image::ImageRgba8(imgbuf).save(fout, image::JPEG);
        },
    }
}

fn copy_to_buffer(buffer : &mut ImageBuffer<Rgba<u8>,Vec<u8>>, img : & mut DynamicImage , start_x : u32, start_y : u32) {
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let start_draw = x >= start_x && y >= start_y;

        if start_draw {
            let x_off = x - start_x;
            let y_off = y - start_y;

            if img.width() > x_off && img.height() > y_off {
                let p = img.get_pixel(x_off, y_off);
                *pixel = p;
            }
        }
    }
}
fn copy_b_to_buffer(buffer : &mut ImageBuffer<Rgba<u8>,Vec<u8>>, img : &mut ImageBuffer<Rgba<u8>,Vec<u8>>, start_x : u32, start_y : u32) {
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let start_draw = x >= start_x && y >= start_y;

        if start_draw {
            let x_off = x - start_x;
            let y_off = y - start_y;

            if img.width() > x_off && img.height() > y_off {
                let p = img.get_pixel(x_off, y_off);
                *pixel = *p;
            }
        }
    }
}

fn update_time(msg : &str, b_time: f64) -> f64 {
    let curr_time = time::precise_time_s();

    println!("{0} -> {1}", msg ,curr_time - b_time);
    curr_time
}

fn qr_generate(url : &[u8]) -> Result<GrayImage, bool  > // QrCode::QrResult<QrCode>
{
    match QrCode::new(url) {
        Err( error ) => {
            println!("{:?}", error);
            //error
            Err(true)
        },
        Ok( result ) =>
        {
            let img : GrayImage = result.render().to_image();
            //img.save(file_name).unwrap();
            Ok(img)
        },
    }
}
