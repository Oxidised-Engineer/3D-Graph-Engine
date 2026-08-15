use crate::DISP_SIZE;
use crate::DrawItem;
use crate::DrawPoint;
use crate::parser::{eval, parser};
use crate::transform::rotate_about_x;
use crate::transform::rotate_about_z;
use chumsky::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use std::io;
use std::ptr::eq;

pub struct Grid {}

pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
const GRID_SIZE: i32 = 100;
const STEP: f32 = 10.0 / GRID_SIZE as f32;
pub fn generate_points() -> Vec<Vec<Point3>> {
    let mut diff_ps: Vec<Vec<Point3>> = Vec::new();
    let mut graphs: Vec<String> = Vec::new();
    let mut still_run = true;
    while still_run {
        let mut equation = String::new();
        io::stdin()
            .read_line(&mut equation)
            .expect("failed to readline");
        //println!("{}", equation);
        let equation: String = equation.trim().to_string();
        if equation == "!" || graphs.len() >= 5 {
            // println!("trigger");
            still_run = false;
        } else {
            graphs.push(equation);
        }
    }

    for src in graphs.iter() {
        let ast = parser().parse(&src.trim()).into_result().unwrap();

        let mut ps: Vec<Point3> = Vec::new();
        for i in -GRID_SIZE..GRID_SIZE {
            for j in -GRID_SIZE..GRID_SIZE {
                let x = i as f32 * STEP;
                let y = j as f32 * STEP;

                let z = eval(&ast, x, y);

                ps.push(Point3 { x: x, y: y, z: z });
            }
        }

        diff_ps.push(ps);
    }
    diff_ps
}

pub fn generate_screen_qs(
    diff_ps: &Vec<Vec<Point3>>,
    phi_z: f64,
    phi_x: f64,
) -> (Vec<Vec<Point3>>, i32) {
    let mut diff_qs: Vec<Vec<Point3>> = Vec::new();
    let y_offset = DISP_SIZE as i32 / 2;

    for ps in diff_ps.iter() {
        let mut qs: Vec<Point3> = Vec::new();
        for i in ps.iter() {
            let px_og = i.x as f32;
            let py_og = i.y as f32;
            let pz_og = i.z as f32;

            let ps_rot_z = rotate_about_z(
                phi_z,
                Point3 {
                    x: px_og,
                    y: py_og,
                    z: pz_og,
                },
            );

            let ps_rot_x = rotate_about_x(
                phi_x,
                Point3 {
                    x: ps_rot_z.x,
                    y: ps_rot_z.y,
                    z: ps_rot_z.z,
                },
            );

            let q_x = (ps_rot_x.x * 15.0) + 250.0;
            let q_y = (-ps_rot_x.z) * 15.0;
            let q_z = ps_rot_x.y;

            qs.push(Point3 {
                x: q_x,
                y: q_y,
                z: q_z,
            });
        }
        diff_qs.push(qs);
    }
    (diff_qs, y_offset)
}

pub fn send_to_display_points(
    diff_qs: &Vec<Vec<Point3>>,
    y_offset: i32,
    colours: &Vec<Vec<f32>>,
) -> Vec<Vec<DrawPoint>> {
    let mut graph_all_items: Vec<Vec<DrawPoint>> = Vec::new();
    let grid_size: usize = (GRID_SIZE * 2) as usize;

    for (count, qs) in diff_qs.iter().enumerate() {
        let colour = &colours[count];
        let mut items: Vec<DrawPoint> = Vec::new();
        let non_nan_z = qs.iter().map(|p| p.z).filter(|z| !z.is_nan());

        let z_min = non_nan_z.clone().fold(f32::INFINITY, f32::min);
        let z_max = non_nan_z.clone().fold(f32::INFINITY, f32::max);

        let mut add_line = |idx1: usize, idx2: usize| {
            let p1 = &qs[idx1];
            let p2 = &qs[idx2];
            if p1.z.is_nan() || p2.z.is_nan() {
                return;
            }
            let avg_z = (p1.z + p2.z) / 2.0;
            let t = ((p1.z - z_min) / (z_max - z_min)).clamp(0.0, 1.0);
            let brightness = 0.2 + 0.8 * t;

            let r = (colour[0] * brightness).clamp(0.0, 255.0) as u8;

            let g = (colour[1] * brightness).clamp(0.0, 255.0) as u8;
            let b = (colour[2] * brightness).clamp(0.0, 255.0) as u8;

            items.push(DrawPoint {
                item: DrawItem::Line {
                    a: Point::new(p1.x as i32, p2.y as i32 + y_offset),
                    b: Point::new(p2.x as i32, p2.y as i32 + y_offset),
                    color: Rgb565::new(r, g, b),
                },
                depth: avg_z,
            });
        };
        for i in 0..grid_size {
            for j in 0..grid_size {
                let current_idx = i * grid_size + j;
                if j < grid_size - 1 {
                    add_line(current_idx, current_idx + 1);
                }
                if i < grid_size - 1 {
                    add_line(current_idx, current_idx + grid_size);
                }
                if i < grid_size - 1 && j < grid_size - 1 {
                    add_line(current_idx, current_idx + grid_size + 1);
                }
            }
        }
        items.sort_by(|a, b| {
            b.depth
                .partial_cmp(&a.depth)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        graph_all_items.push(items);
        //count = count + 1;
    }

    graph_all_items
}
