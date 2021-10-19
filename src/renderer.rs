
#[derive(Default)]
pub struct ParamRenderer {
    
}

impl ParamRenderer {
    pub fn new<W: raw_window_handle::HasRawWindowHandle>(_handle: W) -> Self {
        Self {}
    }
    pub fn draw_frame(&mut self) {
        /* ... */
    }
}
