//extern crate s4_rs;
// https://www.brycevandyk.com/debug-rust-on-windows-with-visual-studio-code-and-the-msvc-debugger/

fn main() {
    let _ = run_game();
}

pub mod types;

use crate::types::{
    camera_controller::CameraController,
    drawable::{DrawParameter, Drawable},
    map::Map,
};
use coffee::{
    graphics::{
        self, Color, Frame, HorizontalAlignment, Mesh, VerticalAlignment, Window, WindowSettings,
    },
    input::KeyboardAndMouse,
    load::{loading_screen::ProgressBar, Task},
    ui::{self, button, Align, Button, Column, Element, Justify, Renderer, Text, UserInterface},
    Game, Result, Timer,
};
use std::time;

pub fn run_game() -> Result<()> {
    <MyGame as UserInterface>::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (600, 600),
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
    last_update: time::Instant,
    interval: time::Duration,
    lmb_down: bool,
    rmb_down: bool,
    player: PlayerInstanceController,
}

pub struct PlayerInstanceController {
    camera: CameraController,
    input: InputController,
}

pub struct InputController {
    last_xpos: u16,
    last_ypos: u16,
}

const TARGET_FPS: u16 = 100;

// https://docs.rs/coffee/0.4.1/coffee/trait.Game.html
impl<'a> Game for MyGame {
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
            map: Map::new(5, 5),
            last_update: time::Instant::now(),
            interval: time::Duration::from_millis((1000 / TARGET_FPS).into()),
            lmb_down: false,
            rmb_down: false,
            player: PlayerInstanceController {
                camera: CameraController {
                    cameraoffset_x: 0,
                    cameraoffset_y: 0,
                    window_height: 0,
                    window_width: 0,
                    mesh: Mesh::new(),
                },
                input: InputController {
                    last_xpos: 0,
                    last_ypos: 0,
                },
            },
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });
        // Reset used mesh
        self.player.camera.mesh = Mesh::new();
        let mut param = DrawParameter {
            camera: &mut self.player.camera,
            frame,
        };
        self.map.draw(&mut param);
        self.player.camera.mesh.draw(&mut frame.as_target());
    }

    fn interact(&mut self, _input: &mut Self::Input, _window: &mut Window) {
        if _input.mouse().is_cursor_within_window() & !_input.mouse().is_cursor_taken() {
            if _input
                .mouse()
                .is_button_pressed(coffee::input::mouse::Button::Left)
            {
                // Left Click
                if !self.lmb_down {
                    self.lmb_down = true;
                    let point = _input.mouse().cursor_position();
                    let h = self.map.tiles[0][0].height as f32;
                    let w = self.map.tiles[0][0].width as f32;
                    let (a, b) = self.map.tiles[0][0].pixel_to_pointy_hex(
                        point.coords.x - (h / 2f32) - self.player.camera.cameraoffset_x as f32,
                        point.coords.y - (w / 2f32) - self.player.camera.cameraoffset_y as f32,
                    );
                    /* // Für Quadrate...
                    let x = usize::from(
                        (((point.coords.x as i16) - self.player.camera.cameraoffset_x) / 30i16)
                            as u16,
                    );
                    let y = usize::from(
                        (((point.coords.y as i16) - self.player.camera.cameraoffset_y) / 30i16)
                            as u16,
                    );*/
                    if (a < self.map.width as i16)
                        & (b < self.map.height as i16)
                        & (a >= 0i16)
                        & (b >= 0i16)
                    {
                        self.map.tiles[a as usize][b as usize].tile_type = rand::random();
                    }
                }
            } else {
                self.lmb_down = false;
            }
            if _input
                .mouse()
                .is_button_pressed(coffee::input::mouse::Button::Right)
            {
                // Click
                if !self.rmb_down {
                    self.rmb_down = true;
                    self.player.input.last_xpos = _input.mouse().cursor_position().coords.x as u16;
                    self.player.input.last_ypos = _input.mouse().cursor_position().coords.y as u16;
                }

                let xdiv: i16 = (_input.mouse().cursor_position().coords.x as i16)
                    - (self.player.input.last_xpos as i16);

                // Check, dass die Karte nicht nach unten rausläuft
                if (self.player.camera.cameraoffset_x + xdiv) <= 0 {
                    // Check, das die Karte nicht nach oben rausläuft (größe + offset + xdiv > Fenster)
                    if ((self.map.width * 30) as i16 + self.player.camera.cameraoffset_x + xdiv)
                        > _window.width() as i16
                    {
                        self.player.camera.cameraoffset_x =
                            self.player.camera.cameraoffset_x + xdiv;
                    }
                }

                let ydiv: i16 = (_input.mouse().cursor_position().coords.y as i16)
                    - (self.player.input.last_ypos as i16);

                // Check, dass die Karte nicht nach rechts rausläuft
                if (self.player.camera.cameraoffset_y + ydiv) <= 0 {
                    // Check, dass die Karte nicht nach links rausläuft
                    if ((self.map.height * 30) as i16 + self.player.camera.cameraoffset_y + ydiv)
                        > _window.height() as i16
                    {
                        self.player.camera.cameraoffset_y =
                            self.player.camera.cameraoffset_y + ydiv;
                    }
                }

                self.player.input.last_xpos = _input.mouse().cursor_position().coords.x as u16;
                self.player.input.last_ypos = _input.mouse().cursor_position().coords.y as u16;
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
        Column::new().into()

        /*let text = format!(
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
            .push(ui::Image::new(&self.image).height(250))
            .push(Button::new(&mut self.increment_button, "+").on_press(Message::IncrementPressed))
            .push(
                Text::new(&self.value.to_string())
                    .size(50)
                    .height(60)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center),
            )
            .push(Button::new(&mut self.decrement_button, "-").on_press(Message::DecrementPressed))
            .into()*/
    }
}
