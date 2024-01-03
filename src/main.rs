extern crate sdl2;

use std::f32::consts::PI;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rand;
use vek::vec::repr_c::vec2::Vec2;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const SCALE: usize = 80;
const CORNERS_Y: usize = HEIGHT / SCALE + 1;
const CORNERS_X: usize = WIDTH / SCALE + 1;

fn normalized_scalar(x: usize) -> f32 {
    return match x {
        0 => 0.,
        v => (v % SCALE) as f32 / SCALE as f32,
    };
}

fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    return (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0;
}

fn pixel_color(x: usize, y: usize, corner_vecs: &[[Vec2<f32>; CORNERS_X]; CORNERS_Y]) -> u8 {
    let left = x / SCALE;
    let top = y / SCALE;
    let right = left + 1;
    let bottom = top + 1;
    let u = normalized_scalar(x);
    let v = normalized_scalar(y);

    let top_left_vec = corner_vecs[top][left];
    let top_right_vec = corner_vecs[top][right];
    let bottom_left_vec = corner_vecs[bottom][left];
    let bottom_right_vec = corner_vecs[bottom][right];

    let top_left_offset = Vec2::new(u, v);
    let top_right_offset = Vec2::new(u-1., v);
    let bottom_left_offset = Vec2::new(u, v-1.);
    let bottom_right_offset = Vec2::new(u-1., v-1.);

    let dot1 = top_left_vec.dot(top_left_offset);
    let dot2 = top_right_vec.dot(top_right_offset);
    let dot3 = bottom_left_vec.dot(bottom_left_offset);
    let dot4 = bottom_right_vec.dot(bottom_right_offset);

    let a1 = interpolate(dot1, dot2, u);
    let a2 = interpolate(dot3, dot4, u);
    let noice_result = interpolate(a1, a2, v);

    return (255. * (noice_result)) as u8;
}

fn main() {
    let bg_color = Color::RGB(0, 0, 0);
    let fg_color = Color::RGB(0, 255, 255);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("fire", WIDTH as u32, HEIGHT as u32).build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut corner_vecs = [[Vec2::new(1., 0.); CORNERS_X]; CORNERS_Y];
    for y in 0..CORNERS_Y {
        for x in 0..CORNERS_X {
            corner_vecs[y][x].rotate_z(PI*2.*rand::random::<f32>());
        }
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                    break 'running
                },
                _ => {},
            }
        }
        canvas.set_draw_color(bg_color);
        canvas.clear();

        for y in 0..CORNERS_Y {
            for x in 0..CORNERS_X {
                corner_vecs[y][x].rotate_z((rand::random::<f32>())*0.05);
            }
        }
        let mut pixels: [u8; WIDTH * HEIGHT * 3] = [0; (WIDTH * HEIGHT * 3)];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let value = pixel_color(x, y, &corner_vecs);
                pixels[y*WIDTH*3 + x*3] = value;
                pixels[y*WIDTH*3 + x*3 + 1] = match value {
                    v if v > 64 => ((v as f32 - 64.) * 1.5) as u8,
                    _ => 0,
                };
                pixels[y*WIDTH*3 + x*3 + 2] = match value {
                    v if v > 128 => (v - 128) * 2,
                    _ => 0,
                };
            }
        }

        let surface = Surface::from_data(&mut pixels, WIDTH as u32, HEIGHT as u32, WIDTH as u32 * 3, PixelFormatEnum::RGB24).unwrap();
        let texture = surface.as_texture(&texture_creator).unwrap();
        canvas.copy(&texture, None, Some(Rect::new(0, 0, WIDTH as u32, HEIGHT as u32))).unwrap();

        canvas.present();
        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
