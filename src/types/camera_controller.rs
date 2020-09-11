use coffee::graphics::Mesh;

pub struct CameraController {
    pub cameraoffset_x: i16,
    pub cameraoffset_y: i16,
    pub window_height: u16,
    pub window_width: u16,
    //frame: Option<&'a mut Frame<'a>>, -> Geht nicht wegen lifetimes.
    pub mesh: Mesh,
}
