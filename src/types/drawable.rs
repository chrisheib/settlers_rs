use coffee::graphics::Frame;

use super::camera_controller::CameraController;

pub trait Drawable {
    fn draw(&mut self, param: &mut DrawParameter);
}

pub struct DrawParameter<'a, 'b, 'c> {
    pub camera: &'b mut CameraController,
    pub frame: &'a mut Frame<'c>,
}
