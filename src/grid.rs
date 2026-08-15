use crate::DrawItem;
use crate::points::Point3;
use crate::transform::{rotate_about_x, rotate_about_z};
use crate::{DISP_SIZE, DrawPoint};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

pub fn generate_initial_grid() -> Vec<Point3> {
    let mut grid: Vec<Point3> = Vec::new();
    let spacing = 3;

    for i in 0..11 {
        for j in 0..11 {
            grid.push(Point3 {
                x: (i * spacing) as f32,
                y: (j * spacing) as f32,
                z: 0.0,
            });
        }
    }
    grid
}

pub fn send_to_display_grid(grid: &Vec<Point3>, phi_z: f64, phi_x: f64) -> Vec<DrawPoint> {
    let mut items: Vec<DrawPoint> = Vec::new();

    let center_z = rotate_about_z(
        phi_z,
        Point3 {
            x: 15.0,
            y: 15.0,
            z: 0.0,
        },
    );
    let center_x = rotate_about_x(phi_x, center_z);

    let cq_x = center_x.x * 15.0;
    let cq_y = -center_x.z * 15.0;
    let y_offset_grid = DISP_SIZE as i32 / 2 - cq_y as i32;
    let x_offset_grid = DISP_SIZE as i32 / 2 - cq_x as i32;
    let mut grid_points: Vec<Point> = Vec::new();
    for p in grid.iter() {
        let ps_rot_z = rotate_about_z(
            phi_z,
            Point3 {
                x: p.x,
                y: p.y,
                z: 0.0,
            },
        );

        let ps_rot_x = rotate_about_x(phi_x, ps_rot_z);

        let q_x = (ps_rot_x.x * 15.0) as i32;
        let q_y = (-ps_rot_x.z * 15.0) as i32;

        grid_points.push(Point::new(q_x + x_offset_grid, q_y + y_offset_grid));
    }

    for i in 0..11 {
        for j in 0..11 {
            let idx = i * 11 + j;
            if i + 1 < 11 {
                let idx_right = (i + 1) * 11 + j;
                items.push(DrawPoint {
                    item: DrawItem::Line {
                        a: grid_points[idx],
                        b: grid_points[idx_right],
                        color: Rgb565::new(255, 255, 255),
                    },
                    depth: 0.0,
                });
            }

            if j + 1 < 11 {
                let idx_down = idx + 1;

                items.push(DrawPoint {
                    item: DrawItem::Line {
                        a: grid_points[idx],
                        b: grid_points[idx_down],
                        color: Rgb565::new(255, 255, 255),
                    },
                    depth: 0.0,
                });
            }
        }
    }
    items
}
