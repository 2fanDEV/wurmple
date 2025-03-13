use std::io::Error;

use configuration::VkConfiguration;
use winit::window::Window;
mod configuration;
mod data;
mod components;

pub struct Engine {
    configuration: VkConfiguration,
}
impl Engine {
    pub fn new(window: &Window) -> Result<Self, Error> {
        let configuration = VkConfiguration::new(window);
        Ok(Self { configuration })
    }
}
