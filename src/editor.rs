use core::ffi::c_void;
use vst::editor::Editor;
use vst_window::{setup, EventSource, WindowEvent};

use crate::renderer::ParamRenderer;

#[derive(Default)]
pub struct ParamEditor {
    renderer: Option<ParamRenderer>,
    window_events: Option<EventSource>
}

impl Editor for ParamEditor {
    fn size(&self) -> (i32, i32) {
        (640, 360)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, parent: *mut c_void) -> bool {
        if self.window_events.is_none() {
            let (window_handle, event_source) = setup(parent, (640, 360));
            self.renderer = Some(ParamRenderer::new(window_handle));
            self.window_events = Some(event_source);
            true
        } else {
            false
        }
    }

    fn is_open(&mut self) -> bool {
        self.window_events.is_some()
    }

    fn close(&mut self) {
        drop(self.renderer.take());
        drop(self.window_events.take());
    }

    fn idle(&mut self) {
        if let Some(window_events) = &mut self.window_events {
            while let Some(event) = window_events.poll_event() {
                match event {
                    WindowEvent::MouseClick(_) => println!("Click!"),
                    _ => (),
                }
            }
        }
        if let Some(renderer) = &mut self.renderer {
            renderer.draw_frame();
        }
    }
}
