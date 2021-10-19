use plugin::ParamTest;
use vst::plugin_main;

mod params;
mod utils;
mod renderer;
mod editor;
mod plugin;

plugin_main!(ParamTest);
