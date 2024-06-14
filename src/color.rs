use nannou::{image::Rgba, math::map_range};
use crate::MAX_COLORS;
pub struct Colorizer(Vec<Rgba<u8>>); 

impl Colorizer {
    pub fn new(num_colors: usize) -> Colorizer {
        Colorizer(
            (0..=num_colors).map(|i| {
                idx2color(i)
                // hsv2rgb(map_range(i, 0, MAX_COLORS, 0, 360))
                // green(i)

            }).collect()    
        )
    }

    // ---

    pub fn get_color(&self, idx: usize) -> Rgba<u8> {
        self.0[idx]
    }
}

// ---

fn idx2color(idx : usize) -> Rgba<u8> {
    // let phi = 1.618033988749895;
    // let n = (idx as f64  * phi - (idx as f64 * phi).floor()) * 360. ;
    // hsv2rgb(n as usize)

    let hue = map_range(idx, 0, MAX_COLORS, 0, 360);
    hsv2rgb(hue)
}

// ---

fn hsv2rgb(h: usize) -> Rgba<u8> {
    // if h <  4 {
    //     return  Rgba([0, 0, 255, 255]);
    // }
    let s: f32 = 1.;
    let v: f32 = 1.;
    let c = s * v;
    let x  = c * (1. - ((h as f32  / 60.) % 2. - 1.).abs());
    let m = v - c;

    let (r, g, b) = 
    if h < 60 {
        (c, x, 0.0)
    } else if h >= 60 && h < 120 {
        (x, c, 0.0)
    } else if h >=  120 && h < 180 {
        (0.0, c, x)
    } else if h >=  180 &&  h < 240 {
        (0.0, x, c)
    } else if h >= 240 && h < 300 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let (r, g, b) = (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    );

    // println!("{}, {:?}", h, (r, g, b));
    Rgba([r, g, b, 255])
}

// ---

