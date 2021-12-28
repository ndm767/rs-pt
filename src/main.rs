mod display;
mod geometry;
mod linalg;
mod ray;
mod scene;

use display::Display;
use geometry::Object;
use geometry::Sphere;
use linalg::Vec3;
use ray::Ray;
use scene::Scene;

fn main() {
    let mut display: Display = Display::new(400, 400);

    let mut scene: Scene = Scene::new(Vec3::new(0.0, 0.0, 0.0));
    scene.add_object(Object::Sphere(Sphere::new(Vec3::new(1.0, 0.0, 2.0), 1.0)));
    scene.add_object(Object::Sphere(Sphere::new(Vec3::new(-1.0, 0.0, 3.0), 0.5)));

    for x in 0..400 {
        for y in 0..400 {
            /* orthographic camera
            let ray = Ray::new(
                Vec3::new((x as f64) / 200.0 - 1.0, (y as f64) / 200.0 - 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            );
            */
            // perspective camera
            let ray = Ray::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new((x as f64) / 200.0 - 1.0, (y as f64) / 200.0 - 1.0, 1.0),
            );
            let hit_ret = ray.trace(&scene);
            display.set_pixel(x, y, hit_ret);
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
