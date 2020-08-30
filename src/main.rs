use rand::distributions::{Distribution, Uniform};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::gfx::primitives::DrawRenderer;

const ROWS: u128 = 100000;
const COLS: u128 = 100000;

fn main() -> Result<(), String> {
    let r = 50000.0;
    let dist = Uniform::from(0..100000 as u128 * 100000 as u128);
    let mut rng = rand::thread_rng();
    let sdl2_context = sdl2::init()?;
    let sdl2_video_subsystem = sdl2_context.video()?;
    let window = sdl2_video_subsystem.window("PI", 600, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut event_pump = sdl2_context.event_pump()?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(15,15,15));
    canvas.clear();
    canvas.circle(300,300,300, Color::WHITE)?;
    let mut points = 0.0;
    let mut circle = 0.0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                _ => {}
            }
        }
        for _ in  0..10000 {
            let pos = dist.sample(&mut rng);
            points += 1.0;
            let x = pos % ROWS;
            let y = pos / ROWS;
            let d = f64::hypot(x as f64 - r, y as f64 - r);
            if d < r {
                circle += 1.0;
                canvas.set_draw_color(Color::GREEN);
            } else {
                canvas.set_draw_color(Color::RED);
            }
            canvas.fill_rect(Rect::new((x * 600 as u128 / ROWS) as i32,
                                       (y * 600 as u128 / COLS) as i32,1,1))?;
        }
        canvas.present();
        canvas.window_mut().set_title(
            &format!("PI: {}", 4.0 * circle / points))
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
