pub(crate) mod vec3;
pub(crate) mod color;

use std::fs::File;
use std::io::prelude::*;
use crate::{
    vec3::{Vec3, Vec3Like},
    color::*
};

fn main() {
    // Image

    let image_width : i32 = 256;
    let image_height : i32 = 256;

    // Render

    let mut file = File::create("images/image.ppm").expect("error in file loading");

    writeln!(file, "P3\n{0} {1}\n255", image_width, image_height).expect("Couldn't write in file");

    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height-j);
        for i in 0..image_width {
            let pixel_color = Color::from_xyz((i as f64)/((image_width-1) as f64), (j as f64)/((image_height-1) as f64), 0.0);

            write_color(&file, &pixel_color);
        }
    }
    println!("Done !");
}