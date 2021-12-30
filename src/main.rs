mod camera;
mod display;
mod geometry;
mod linalg;
mod materials;
mod ray;
mod scene;

use camera::Camera;
use display::Display;
use display::Keycode;
use geometry::Object;
use geometry::Sphere;
use linalg::Vec3;
use materials::Lambertian;
use materials::Material;
use materials::PerfectSpecular;
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

    let mut camera: Camera = Camera::new(Vec3::new(0.0, 0.0, 0.0));
    let mut did_move: bool = false;

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
                let ray = camera.gen_ray(x, y, display.width, display.height);
                let hit_ret = ray.trace(&scene);
                if !did_move {
                    display.set_pixel(x, y, hit_ret);
                } else {
                    display.reset_pixel(x, y, hit_ret);
                }
            }
        }

        did_move = false;

        display.show();

        display.poll_events();

        if display.are_keys_down() {
            // translations
            if display.is_key_down(Keycode::W) {
                camera.translate_in_dir(0.1, 0.0, 0.0);
                did_move = true;
            }

            if display.is_key_down(Keycode::S) {
                camera.translate_in_dir(-0.1, 0.0, 0.0);
                did_move = true;
            }

            if display.is_key_down(Keycode::A) {
                camera.translate_in_dir(0.0, -0.1, 0.0);
                did_move = true;
            }

            if display.is_key_down(Keycode::D) {
                camera.translate_in_dir(0.0, 0.1, 0.0);
                did_move = true;
            }

            if display.is_key_down(Keycode::E) {
                camera.translate(0.0, -0.1, 0.0);
                did_move = true;
            }

            if display.is_key_down(Keycode::Q) {
                camera.translate(0.0, 0.1, 0.0);
                did_move = true;
            }

            // rotations
            if display.is_key_down(Keycode::Left) {
                camera.rotate(0.0, -0.1, 0.0);
                did_move = true;
            }

            if display.is_key_down(Keycode::Right) {
                camera.rotate(0.0, 0.1, 0.0);
                did_move = true;
            }

            if display.is_key_down(Keycode::Up) {
                camera.rotate(0.1, 0.0, 0.0);
                did_move = true;
            }
            if display.is_key_down(Keycode::Down) {
                camera.rotate(-0.1, 0.0, 0.0);
                did_move = true;
            }
        }

        if display.should_quit {
            break;
        }
    }
}
