//mod main{}

use coffee::graphics::{self, Color, Frame, HorizontalAlignment, VerticalAlignment, Window, WindowSettings,};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use coffee::ui::{button, Button, Align, Column, Element, Image, Justify, Renderer, Text, UserInterface,};

fn main() -> Result<()> {
    <MyGame as UserInterface>::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

struct MyGame {
    // Your game state and assets go here...
    image: graphics::Image,
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

impl Game for MyGame {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        // Load your game assets here. Check out the `load` module!
        graphics::Image::load("res\\img\\example.png")
            .map(|image| MyGame {
                image,
                value: 0,
                increment_button: button::State::new(),
                decrement_button: button::State::new(),
            })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });
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
        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .align_items(Align::Center)
            .justify_content(Justify::Center)
            .spacing(20)
            .push(
                Text::new("This is an image")
                    .size(50)
                    .height(60)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center),
            )
            .push(Image::new(&self.image).height(250))
            .push(
                Button::new(&mut self.increment_button, "+")
                    .on_press(Message::IncrementPressed),
            )
            .push(
                Text::new(&self.value.to_string())
                    .size(50)
                    .height(60)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center),
            )
            .push(
                Button::new(&mut self.decrement_button, "-")
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}