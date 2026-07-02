use std::path::Path;

use vtracer::{ColorMode, Config, convert_image_to_svg};

pub fn image_to_svg(   
) {    let config = Config {
        color_mode: ColorMode::Color,
        path_precision:Some(88),
        ..Default::default() 
    };  let mut on =  convert_image_to_svg(
    Path::new("assets/fractal.png"),
    Path::new("assets/fractal.svg"),
    config,
);   }