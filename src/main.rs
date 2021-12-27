mod display;
mod geometry;
mod linalg;
mod ray;

use display::Display;
use geometry::Hittable;
use geometry::Sphere;
use linalg::Vec3;
use ray::Ray;

fn main() {
    let mut display: Display = Display::new(400, 400);

    let sphere: Sphere = Sphere::new(Vec3::new(0.0, 0.0, 2.0), 1.0);

    for x in 0..400 {
        for y in 0..400 {
            let ray = Ray::new(
                Vec3::new((x as f64) / 200.0 - 1.0, (y as f64) / 200.0 - 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            );
            let hit_ret = sphere.calc_intersection(ray);
            if hit_ret.is_none() {
                display.set_pixel(x, y, Vec3::new(0.0, 0.0, 0.0));
            } else {
                display.set_pixel(x, y, Vec3::new(1.0, 1.0, 1.0));
            }
        }
    }
    loop {
        display.show();

        display.poll_events();

        if display.should_quit {
            break;
        }
    }
}
