use nannou::{
    prelude::*,
    image::ImageBuffer,  wgpu::*
};

use crate::color::Colorizer;

pub const MAX_COLORS : usize = 360;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::wait())
        .run();
}
mod color;
struct Model {
    start:(f64, f64), 
    step: (f64, f64),
    texture: Texture,
    colorizer: color::Colorizer,
}

fn model(app: &App) -> Model {
    let x_bounds =  (-2., 1.); 
    let y_bounds =  (-1.5, 1.5);
    let win_w = 800.;
    let win_h = ((y_bounds.1 - y_bounds.0) / (x_bounds.1 - x_bounds.0)) * win_w;
    
    app.new_window()
    .size(win_w as u32, win_h as u32)
    .view(view)
    .mouse_released(mouse_released)
    .build()
    .unwrap();

    let win = app.main_window();
    let dim = win.rect().w_h();   
    Model { 
        start: (x_bounds.0, y_bounds.0),
        step: ((x_bounds.1 - x_bounds.0) / dim.0 as f64,  (y_bounds.1 - y_bounds.0) / dim.1 as f64),
        texture: TextureBuilder::new()
            .size([dim.0 as u32, dim.1 as u32])
            .format(TextureFormat::Rgba8Unorm)
            .usage(TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING)
            .build(win.device()),
        colorizer: Colorizer::new(MAX_COLORS),
    }
}

// ---

fn calc(x : f64, y : f64) -> usize {
    let mut lx = x;
    let mut ly = y;
    let mut n = 0;
    let ly2 = ly * ly;
    let lx2 = lx * lx;

    // cardioid check
    let q = lx2 - 0.5 * lx  + 0.0625 + ly2;
    if ly2  >= 4.0 * q * (q + lx - 0.25) {
         return 0;
    }
    // bulb check
    if (lx + 1.0) * (lx + 1.0) + ly2 < 0.0625 {
         return 0;
    }

    while  (lx * lx + ly * ly  < 4.0) && (n < MAX_COLORS) {
        let lxt = lx * lx - ly * ly + x;
        ly = 2. * lx * ly + y;
        lx = lxt;
        n += 1;
    }

    if n == MAX_COLORS {0} else {n}
}

// ---

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let dim = app.window_rect().w_h();
    let image = ImageBuffer::from_fn(
        dim.0 as u32,
        dim.1 as u32,
        |x, y| {
            let idx = calc(
                model.start.0 + x as f64 * model.step.0, 
                model.start.1 + y as f64 * model.step.1
            );
            model.colorizer.get_color(idx)
        },
    );
    let flat_samples = image.as_flat_samples();
    model.texture.upload_data(
        app.main_window().device(),
        &mut frame.command_encoder(),
        flat_samples.as_slice(),
    );
    draw.texture(&model.texture);
    frame.clear(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

// ---

fn mouse_released(app: &App, model: &mut Model, button: MouseButton) {
    if ! [MouseButton::Left, MouseButton::Right].contains(&button) {
        return; 
    }
    let w_h = app.window_rect().w_h();
    let frame_ratio = 10.;
    let app_mouse = (app.mouse.x, app.mouse.y);
    let phys_mouse = (app_mouse.0 + w_h.0 / 2.,  w_h.1 / 2. - app_mouse.1); 

    if button == MouseButton::Left {
        let half_frame_w = (w_h.0 / frame_ratio) / 2.;
        let half_frame_h = (w_h.1 / frame_ratio) / 2.;
        let lt =  ((phys_mouse.0 - half_frame_w) as f64, (phys_mouse.1 - half_frame_h) as f64);
        model.start = (model.start.0 + lt.0 * model.step.0, model.start.1 + lt.1 * model.step.1);
        model.step =  (model.step.0 / frame_ratio as f64, model.step.1 / frame_ratio as f64);
    } else if button == MouseButton::Right {
        let point  =  (model.start.0 + phys_mouse.0 as f64  * model.step.0, model.start.1 + phys_mouse.1 as f64 * model.step.1);
        model.step =  (model.step.0 *  frame_ratio as f64, model.step.1 * frame_ratio as f64);
        model.start = (point.0 - model.step.0 * w_h.0 as f64 / 2., point.1 - model.step.1 * w_h.1 as f64 / 2.);
    }
}