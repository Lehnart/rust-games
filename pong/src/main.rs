use std::time::SystemTime;

use audio::Audio;
use collide::Collide;
use engine::audio::init_audio;
use engine::graphics::Window;
use event::handle_event;
use logic::Logic;

use crate::graphics::Graphics;

mod graphics;
mod event;
mod collide;
mod audio;
mod logic;

fn main() {
    init_audio(4);
    let audio = Audio::new();
    let mut logic = Logic::new(&audio);
    let mut collide = Collide::new(&audio);

    let mut window = Window::new(600, 600);
    let ttf_context = sdl2::ttf::init().unwrap();

    let mut graphics = Graphics::new();


    let mut previous = SystemTime::now();
    'game_loop: loop {
        let next = SystemTime::now();
        let dt = next.duration_since(previous).unwrap().as_secs_f32();
        previous = next;

        let event_pump = &mut window.event_pump;
        for event in event_pump.poll_iter() {
            handle_event(event, &mut logic);
        }

        logic.update(dt);
        if logic.is_over() {
            break 'game_loop;
        }

        collide.collide_ball_and_wall(&mut logic);
        collide.collide_ball_and_racket(&mut logic);

        graphics.update(&logic, &window);
        graphics.draw(&mut window, &ttf_context);
    }
}

