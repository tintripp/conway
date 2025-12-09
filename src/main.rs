extern crate sdl3;

use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::keyboard::Scancode;
use sdl3::render::Canvas;
use sdl3::render::FRect;
use sdl3::render::RenderTarget;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl3 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let player_speed = 5;

    struct Vector2 {
        x: f32,
        y: f32,
    }

    impl Vector2 {
        fn get_magnitude(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }

        fn normalized(&self) -> (f32, f32) {
            (
                self.x / self.get_magnitude(),
                self.y / self.get_magnitude()
            )
        }
    }

    struct Player {
        rect: FRect,
        velocity: Vector2,
        color: Color,
    }

    impl Player {
        fn draw<T: RenderTarget>(&self, canvas: &mut Canvas<T>) {
            canvas.set_draw_color(self.color);
            canvas.fill_rect(self.rect).unwrap();
        }
    }

    let mut canvas = window.into_canvas();

    let mut player: Player = Player {
        rect: FRect::new(64_f32, 64_f32, 128_f32, 128_f32),
        velocity: Vector2 { x: 0_f32, y: 0_f32 },
        color: Color::RGB(67, 129, 67)
    };

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        // Event Loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Update Loop



        let ks = event_pump.keyboard_state();
        for sc in ks.pressed_scancodes() {
            // println!("{}", sc.name());

            match sc {
                Scancode::W => player.velocity.y -= player_speed as f32,
                Scancode::S => player.velocity.y += player_speed as f32,
                Scancode::A => player.velocity.x -= player_speed as f32,
                Scancode::D => player.velocity.x += player_speed as f32,
                _ => {}
            }
        }
        //dampen player velocity
        player.velocity.x *= 0.85;
        player.velocity.y *= 0.85;

        /*println!("{}",player.velocity.normalized().0);

        player.rect.x += player.velocity.normalized().0;
        player.rect.y += player.velocity.normalized().1;*/

        let (window_w, window_h) = canvas.window().size();
        let window_w = window_w as f32;
        let window_h = window_h as f32;
        
        if player.rect.x <= 0_f32 {
            player.rect.x = 0_f32;
        }else if player.rect.x >= window_w - player.rect.w {
            player.rect.x = window_w - player.rect.w;
        }
        if player.rect.y <= 0_f32 {
            player.rect.y = 0_f32;
        }else if player.rect.y >= window_h - player.rect.h {
            player.rect.y = window_h - player.rect.h;
        }

        // Draw
        player.draw(&mut canvas);


        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}