mod display;
mod geometry;
mod linalg;
mod materials;
mod ray;
mod scene;

use display::Display;
use geometry::Object;
use geometry::Sphere;
use linalg::Vec3;
use materials::Lambertian;
use materials::Material;
use materials::PerfectSpecular;
use ray::Ray;
use scene::Scene;

fn main() {
    let mut display: Display = Display::new(400, 400);

    let mut scene: Scene = Scene::new(Vec3::new(0.0, 0.0, 0.0));
    let red: Material = Material::Lambertian(Lambertian::new(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        0.9,
    ));
    let white: Material =
        Material::PerfectSpecular(PerfectSpecular::new(Vec3::new(1.0, 1.0, 1.0), 0.9));
    scene.add_object(Object::Sphere(Sphere::new(
        Vec3::new(1.0, 0.0, 2.0),
        1.0,
        red,
    )));
    scene.add_object(Object::Sphere(Sphere::new(
        Vec3::new(-0.5, 0.0, 2.0),
        0.5,
        white,
    )));

    loop {
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
                    0,
                    1.0,
                );
                let hit_ret = ray.trace(&scene);
                display.set_pixel(x, y, hit_ret);
            }
        }
        display.show();

        display.poll_events();

        if display.should_quit {
            break;
        }
    }
}
