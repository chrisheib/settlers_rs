//extern crate s4_rs;
// https://www.brycevandyk.com/debug-rust-on-windows-with-visual-studio-code-and-the-msvc-debugger/

fn main() {
    let _ = run_game();
}

pub mod types;
//pub mod types::tile;
//pub mod types::map;

use coffee::graphics::{
    self, Color, Frame, HorizontalAlignment, VerticalAlignment, Window, WindowSettings,
};

use coffee::load::{loading_screen::ProgressBar, Task};
use coffee::ui::{
    button, Align, Button, Column, Element, Image, Justify, Renderer, Text, UserInterface,
};
use coffee::{Game, Result, Timer, input::KeyboardAndMouse};
use types::map::Map;
use std::{time};

pub fn run_game() -> Result<()> {
    <MyGame as UserInterface>::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })

}

pub struct MyGame {
    // Your game state and assets go here...
    image: graphics::Image,
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    map: Map,
    last_update : time::Instant,
    interval : time::Duration,
    lmb_down : bool,
    rmb_down : bool,
    last_xpos : u16,
    last_ypos : u16,
    cameraoffset_x : i16,
    cameraoffset_y : i16,
}

const TARGET_FPS: u16 = 100;

// https://docs.rs/coffee/0.4.1/coffee/trait.Game.html
impl Game for MyGame{
    const TICKS_PER_SECOND: u16 = 200;
    type Input = KeyboardAndMouse;
    type LoadingScreen = ProgressBar; // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        // Load your game assets here. Check out the `load` module!
        graphics::Image::load("res\\img\\example.png").map(|image| MyGame {
            image,
            value: 0,
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
            map: Map::new(100, 100),
            last_update : time::Instant::now(),
            interval : time::Duration::from_millis((1000 / TARGET_FPS).into()),
            lmb_down : false,
            rmb_down : false,
            last_xpos : 0,
            last_ypos : 0,
            cameraoffset_x : 0,
            cameraoffset_y : 0,
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        if _timer.has_ticked(){
            if self.last_update.elapsed() > self.interval{
                self.last_update = time::Instant::now();
                frame.clear(Color {
                    r: 0.3,
                    g: 0.3,
                    b: 0.6,
                    a: 1.0,
                });
                self.map.draw_map(frame, &self.cameraoffset_x, &self.cameraoffset_y);
            } else {
                //thread::sleep(self.interval - self.last_update.elapsed() - time::Duration::from_millis(1));
            }
        }
    }

    fn interact(&mut self, _input: &mut Self::Input, _window: &mut Window) {
        if _input.mouse().is_cursor_within_window() & !_input.mouse().is_cursor_taken(){
            if _input.mouse().is_button_pressed(coffee::input::mouse::Button::Left){
                // Left Click
                if !self.lmb_down {
                    self.lmb_down = true;
                    let point = _input.mouse().cursor_position();
                    let x = usize::from((((point.coords.x as i16) - self.cameraoffset_x) / 30i16) as u16);
                    let y = usize::from((((point.coords.y as i16) - self.cameraoffset_y) / 30i16) as u16);
                    if (x < usize::from(self.map.width)) & (y < usize::from(self.map.height)) {
                        self.map.tiles[x][y].tile_type = rand::random();
                    }
                }
            } else {
                self.lmb_down = false;
            }
            if _input.mouse().is_button_pressed(coffee::input::mouse::Button::Right){
                // Click
                if !self.rmb_down {
                    self.rmb_down = true;
                    self.last_xpos = _input.mouse().cursor_position().coords.x as u16;
                    self.last_ypos = _input.mouse().cursor_position().coords.y as u16;
                }
                let xdiv: i16 = (_input.mouse().cursor_position().coords.x as i16) - (self.last_xpos as i16);
                if (self.cameraoffset_x + xdiv) <= 0 {
                    self.cameraoffset_x = self.cameraoffset_x + xdiv;
                }
                let ydiv: i16 = (_input.mouse().cursor_position().coords.y as i16) - (self.last_ypos as i16);
                if (self.cameraoffset_y + ydiv) <= 0 {
                    self.cameraoffset_y = self.cameraoffset_y + ydiv;
                }
                self.last_xpos = _input.mouse().cursor_position().coords.x as u16;
                self.last_ypos = _input.mouse().cursor_position().coords.y as u16;
            } else {
                self.rmb_down = false;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl UserInterface for MyGame {
    type Message = Message;
    type Renderer = Renderer;

    fn react(&mut self, message: Message, _window: &mut Window) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn layout(&mut self, window: &Window) -> Element<Message> {
        let text = format!(
            "This is an image {} {}",
            self.map.height.to_string(),
            self.map.width.to_string()
        );
        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .align_items(Align::Center)
            .justify_content(Justify::Center)
            .spacing(20)
            .push(
                Text::new(&text)
                    .size(50)
                    .height(60)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center),
            )
            .push(Image::new(&self.image).height(250))
            .push(Button::new(&mut self.increment_button, "+").on_press(Message::IncrementPressed))
            .push(
                Text::new(&self.value.to_string())
                    .size(50)
                    .height(60)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center),
            )
            .push(Button::new(&mut self.decrement_button, "-").on_press(Message::DecrementPressed))
            .into()
    }
}
