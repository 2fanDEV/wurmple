use ash::{ext::debug_utils, vk::DebugUtilsMessengerEXT, Entry, Instance};
use winit::window::Window;

use super::components::{self, create_debugger, create_entry_and_instance};
pub struct VkConfiguration {
    entry: Entry,
    instance: Instance,
    debug_instance: debug_utils::Instance,
    debugger: DebugUtilsMessengerEXT,
}

impl VkConfiguration {
    pub fn new(window: &Window) -> Self {
        let (entry, instance) = create_entry_and_instance(window);
        let (debug_instance, debugger) = create_debugger(&entry, &instance);
        Self {
            entry,
            instance,
            debug_instance,
            debugger,
        }
    }

    pub fn destroy(&self) {
        unsafe { self.instance.destroy_instance(None) };
    }
}
