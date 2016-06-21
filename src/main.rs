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

const IMAGE_SIZE: (u32,u32) = (1728, 2592);
const PRINT_SIZE: (u32,u32) = (1205, 1795);
const PRINT_SIZE_CENTER: (u32,u32) = (PRINT_SIZE.0 / 2, PRINT_SIZE.1 / 2);

const QR_MESURE: u32 = 8;

#[allow(unused_variables)]
fn main() {
    let starting_time = time::precise_time_s();

    let mut img = image::open(&Path::new("belen.jpg")).unwrap();

    let f_x = 60;
    let f_y = -55;

    let small_size = (602, 1070); //non crop 602, 1070 with crop 602,709
    let icao_size = (0, 180, 602, 889); // crop 2522 -> 2304
    let last_size = (0 + f_x, 180 + f_y, 602 + f_x, 889 + f_y);

    //img = img.rotate90().crop(0, 180, 602, 889);

    //image4x(&mut img);
    base_image(&mut img);

    update_time("Running", starting_time);
}

#[allow(dead_code,unused_variables, unused_mut)]
fn base_image(mut img : &mut DynamicImage) {
    let mut imgbuf : ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(PRINT_SIZE_CENTER.0, PRINT_SIZE_CENTER.0);

    match qr_generate(b"test-url") {
        Err( err ) => println!("QR render error {:?}", err),
        Ok( qr_render ) => {
            let mut qr_rgba : ImageBuffer<Rgba<u8>, Vec<u8>> = qr_render.convert();

            let qr_w = qr_rgba.width();
            let qr_h = qr_rgba.height();

            *img = img.resize(PRINT_SIZE_CENTER.0,PRINT_SIZE_CENTER.1, image::FilterType::Nearest);
            *img = img.rotate90();

            copy_to_buffer(&mut imgbuf, &mut img, 0, 0);
            copy_b_to_buffer(&mut imgbuf, &mut qr_rgba , PRINT_SIZE_CENTER.0 - qr_w , PRINT_SIZE_CENTER.1 - qr_h);

            let ref mut fout = File::create(&Path::new("base_example.jpg")).unwrap();
            let _ = image::ImageRgba8(imgbuf).save(fout, image::JPEG);
        },
    }
}

#[allow(dead_code,unused_variables, unused_mut)]
fn image4x(img : &mut DynamicImage) {
    let mut imgbuf : ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(1205, 1795);

    match qr_generate(b"test-url") {
        Err( err ) => println!("QR render error {:?}", err),
        Ok( qr_render ) =>
        {
            let mut qr_rgba : ImageBuffer<Rgba<u8>, Vec<u8>>= qr_render.convert();

            copy_to_buffer(&mut imgbuf, img, 0 , 0);
            copy_to_buffer(&mut imgbuf, img, 500 , 500);

            copy_b_to_buffer(&mut imgbuf, &mut qr_rgba , 0 , PRINT_SIZE_CENTER.1);
            copy_b_to_buffer(&mut imgbuf, &mut qr_rgba , PRINT_SIZE_CENTER.0 , PRINT_SIZE_CENTER.1);

            let ref mut fout = File::create(&Path::new("x4_example.jpg")).unwrap();
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
            let img : GrayImage = result.render().module_size(QR_MESURE).to_image();
            //img.save(file_name).unwrap();
            Ok(img)
        },
    }
}
