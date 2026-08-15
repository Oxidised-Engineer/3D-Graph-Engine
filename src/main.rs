pub mod display;
pub mod grid;
pub mod parser;
pub mod points;
pub mod transform;
use crate::display::display_all;
use crate::points::{generate_points, generate_screen_qs, send_to_display_points};
use crate::{
    grid::{generate_initial_grid, send_to_display_grid},
    points::Point3,
};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;
use std::time::Instant;
use std::{thread, time::Duration};

pub struct DrawPoint {
    pub item: DrawItem,
    pub depth: f32,
}

pub enum DrawItem {
    Rect {
        pos: Point,
        size: u32,
        color: Rgb565,
    },
    Line {
        a: Point,
        b: Point,
        color: Rgb565,
    },
}

const DISP_SIZE: u32 = 500;
fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(DISP_SIZE, DISP_SIZE));
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Graph", &output_settings);

    let ps: Vec<Vec<Point3>> = generate_points();
    let grid: Vec<Point3> = generate_initial_grid();

    let mut phi_z: f64 = 30.0;
    let mut phi_x: f64 = 30.0;
    let colours = vec![
        vec![235.0, 135.0, 135.0],
        vec![0.0, 0.0, 31.0],
        vec![31.0, 63.0, 0.0],
        vec![0.0, 63.0, 31.0],
        vec![31.0, 50.0, 29.0],
    ];
    'running: loop {
        let _ = display.clear(Rgb565::new(1, 1, 1));
        let compute_start = Instant::now();

        let generated_screen_qs = generate_screen_qs(&ps, phi_z, phi_x);

        let mut diff_qs: Vec<Vec<Point3>> = generated_screen_qs.0;
        let y_offset = generated_screen_qs.1;

        let mut items: Vec<Vec<DrawPoint>> = Vec::new();

        items.push(send_to_display_grid(&grid, phi_z, phi_x));

        let graph_items = send_to_display_points(&mut diff_qs, y_offset, &colours);

        items.extend(graph_items);
        let mut flattened_items: Vec<DrawPoint> = items.into_iter().flatten().collect();

        display_all(&mut flattened_items, &mut display);
        diff_qs.clear();

        flattened_items.clear();

        window.update(&display);

        let compute_time = compute_start.elapsed();
        //println!("{:?}", compute_time);
        for e in window.events() {
            match e {
                SimulatorEvent::Quit => {
                    break 'running Ok(());
                }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Left => {
                        phi_z += 5.0;
                    }
                    Keycode::Right => {
                        phi_z -= 5.0;
                    }
                    Keycode::Up => phi_x += 5.0,
                    Keycode::Down => phi_x -= 5.0,
                    Keycode::Escape => break 'running Ok(()),
                    _ => {}
                },
                _ => {}
            }
        }
        thread::sleep(Duration::from_millis(1));
    }
}
