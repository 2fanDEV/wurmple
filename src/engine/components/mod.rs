use ash::{ext::debug_utils, vk::{DebugUtilsMessengerEXT, PhysicalDevice}, Device, Entry, Instance};
use instance::{create_instance, load_vulkan_library};
use winit::window::Window;

mod instance;
mod device;
pub fn create_entry_and_instance(window: &Window) -> (Entry, Instance) {
    let entry = load_vulkan_library().unwrap();
    let instance = create_instance(&entry, window).unwrap();
    (entry, instance)
}

pub fn create_debugger(entry: &Entry, instance: &Instance) -> (debug_utils::Instance, DebugUtilsMessengerEXT) {
    instance::create_debugger(&entry, &instance)
}

pub fn create_device(instance: &Instance) -> (PhysicalDevice, Device) {

}
