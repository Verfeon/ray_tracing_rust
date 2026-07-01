use crate::vec3::Vec3Like;
use crate::vec3::impl_vec3_ops;
use std::io::Write;

pub struct Color {
    pub(crate) r: f64,
    pub(crate) g: f64,
    pub(crate) b: f64
}

impl Vec3Like for Color {
    fn x(&self) -> f64 {self.r}
    fn y(&self) -> f64 {self.g}
    fn z(&self) -> f64 {self.b}

    fn from_xyz(x: f64, y: f64, z: f64) -> Color { Color {r: x , g: y, b: z} }
}

impl_vec3_ops!(Color);

pub fn write_color(mut f: &std::fs::File, color: &Color) {
    let r_byte: i32 = (color.r * 255.999) as i32;
    let g_byte: i32 = (color.g * 255.999) as i32;
    let b_byte: i32 = (color.b * 255.999) as i32;

    writeln!(f, "{0} {1} {2}", r_byte, g_byte, b_byte).expect("Couldn't write in file");
}