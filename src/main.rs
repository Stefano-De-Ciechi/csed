/*
* author: Stefano De Ciechi
* purpose: csed - Colored Sobel Edge Detection on images
* date: 2024-07-18
*/

use image::{open, DynamicImage, GenericImage, GenericImageView, Rgba};
use clap::{Command, Arg};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

fn main() {
    
    let args = Command::new("CSED - Colored Sobel Edge Detection")
        .version("0.1")
        .about("apply the Sobel Edge Detection algorithm with colors to an image")
        .arg(Arg::new("input")
            .required(true)
            .help("path of the input image")
        )

        .arg(Arg::new("output")
            .short('o')
            .default_value("./")
            .help("output path (a folder)")
        )

        .get_matches();


    let img_path: &String = args.get_one("input").unwrap();
    let img_path: PathBuf = PathBuf::from_str(img_path).unwrap();

    let img_name = img_path.file_name().unwrap();
    println!("img_name: {}", img_name.to_str().unwrap());

    let output_path: &String = args.get_one("output").unwrap();
    let output_path: PathBuf = PathBuf::from_str(output_path).unwrap();

    if !output_path.is_dir() || !output_path.exists() {
        eprintln!("output is expected to be an existing directory");
        exit(-1);
    }

    let img: DynamicImage = open(&img_path).expect("Failed to open image");
    let out_img: DynamicImage = sobel_edge_detector(img);

    let output_img_name = format!("sobel_{}", img_name.to_str().unwrap());
    let output_path = output_path.join(output_img_name);

    out_img.save(output_path)
        .expect("error saving the image");

}

fn sobel_edge_detector(image: DynamicImage) -> DynamicImage {
    static SOBEL_X: [[i32; 3]; 3] = [
        [1, 0, -1],
        [2, 0, -2],
        [1, 0, -1],
    ];

    static SOBEL_Y: [[i32; 3]; 3] = [
        [1, 2, 1],
        [0, 0, 0],
        [-1, -2, -1],
    ];

    let (img_w, img_h) = image.dimensions();
    let mut output = DynamicImage::new_rgb8(img_w, img_h);

    for y in 1..img_h - 1 {
        for x in 1..img_w - 1 {
            let mut r_x = 0;
            let mut g_x = 0;
            let mut b_x = 0;

            let mut r_y = 0;
            let mut g_y = 0;
            let mut b_y = 0;

            for ky in 0..3 as usize {
                for kx in 0..3 as usize {
                    let pixel = image.get_pixel(x + kx as u32 - 1, y + ky as u32 - 1);
                    let r = pixel[0] as i32;
                    let g = pixel[1] as i32;
                    let b = pixel[2] as i32;

                    r_x += r * SOBEL_X[ky][kx];
                    g_x += g * SOBEL_X[ky][kx];
                    b_x += b * SOBEL_X[ky][kx];

                    r_y += r * SOBEL_Y[ky][kx];
                    g_y += g * SOBEL_Y[ky][kx];
                    b_y += b * SOBEL_Y[ky][kx];
                }
            }

            let r = ((r_x * r_x + r_y * r_y) as f64).sqrt() as u8;
            let g = ((g_x * g_x + g_y * g_y) as f64).sqrt() as u8;
            let b = ((b_x * b_x + b_y * b_y) as f64).sqrt() as u8;

            output.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }

    output
}
