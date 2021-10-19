
use vst::{editor::Editor, plugin::{HostCallback, Info, Plugin}};

use crate::editor::ParamEditor;

#[derive(Default)]
pub struct ParamTest {
    host: HostCallback,
    editor_placeholder: Option<ParamEditor>
}

impl Plugin for ParamTest {
    fn new(host: HostCallback) -> ParamTest {
        return ParamTest {
            host: host,
            editor_placeholder: Some(ParamEditor::default())
        };
    }
    
    fn get_info(&self) -> Info {
        return Info {
            name: "ParamTest".to_string(),
            
            unique_id: 42069,
            
            inputs: 0,
            
            outputs: 2,
            
            
            
            ..Default::default()
        };
    }
    
    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        
    }
    
    fn process_events(&mut self, events: &vst::api::Events) {
        
    }
    
    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        self.editor_placeholder
            .take()
            .map(|editor| Box::new(editor) as Box<dyn Editor>)
    }
}

