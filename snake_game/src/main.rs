extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::*;

use std::collections::LinkedList;

struct Snake {
    pos_x: i32,
    pos_y: i32,
    dir: Direction,
}

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let BLACK: [f32; 4] = [1.0, 1.0, 1.0, 0.8];

        let square =
            graphics::rectangle::square((self.pos_x * 20) as f64, (self.pos_y * 20) as f64, 20_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(BLACK, square, transform, gl)
        })
    }

    fn update(&mut self) {
        match self.dir {
            Direction::Left => self.pos_x -= 1,
            Direction::Right => self.pos_x += 1,
            Direction::Up => self.pos_y -= 1,
            Direction::Down => self.pos_y += 1,
        }
    }
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        let WHITE: [f32; 4] = [0.0, 0.0, 0.0, 0.8];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(WHITE, gl);
        });

        self.snake.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => Direction::Right,
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Window", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            pos_x: 0,
            pos_y: 0,
            dir: Direction::Right,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r)
        }

        if let Some(_) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button)
            }
        }
    }
}
