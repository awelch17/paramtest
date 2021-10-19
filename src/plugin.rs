
use vst::plugin::{HostCallback, Info, Plugin};

#[derive(Default)]
pub struct ParamTest {
    host: HostCallback
}

impl Plugin for ParamTest {
    fn new(host: HostCallback) -> ParamTest {
        return ParamTest {
            host: host
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
    
}

