mod display;
mod linalg;

use display::Display;

fn main() {
    let mut display: Display = Display::new(400, 400);

    for x in 0..400 {
        for y in 0..400 {
            let color = linalg::Vec3::new((x as f64) / 400.0, (y as f64) / 400.0, 0.0);
            display.set_pixel(x, y, color);
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
