use std::{
    env,
    ffi::c_void,
    fmt::Error,
    ops::Add,
};

use ash::{
    ext::debug_utils,
    vk::{
        ApplicationInfo, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
        DebugUtilsMessengerCallbackDataEXT, DebugUtilsMessengerCreateInfoEXT,
        DebugUtilsMessengerEXT, InstanceCreateFlags, InstanceCreateInfo, API_VERSION_1_2,
        EXT_DEBUG_UTILS_NAME,
    },
    Entry, Instance,
};
use log::{debug, error, info, warn};
use winit::{raw_window_handle::HasDisplayHandle, window::Window};


pub fn load_vulkan_library() -> Result<Entry, Error> {
    #[cfg(target_os = "macos")]
    let entry_path = env::home_dir().unwrap().to_str().unwrap().to_owned()
        + "/VulkanSDK/1.4.309.0/macOS/lib/libvulkan.dylib";
    Ok(unsafe { Entry::load_from(entry_path).unwrap() })
}

pub fn create_instance(entry: &Entry, window: &Window) -> Result<Instance, Error> {
    let engine_name = c"ELPMRUW";
    let application_name = c"WURMPLE";
    let application_info = ApplicationInfo::default()
        .engine_name(engine_name)
        .api_version(API_VERSION_1_2)
        .application_name(application_name);
    let mut required_extensions =
        ash_window::enumerate_required_extensions(window.display_handle().unwrap().as_raw())
            .unwrap()
            .to_vec();
    required_extensions.push(ash::khr::portability_enumeration::NAME.as_ptr());
    let extension_properties = unsafe {
        entry
            .enumerate_instance_extension_properties(None)
            .unwrap()
            .iter()
            .map(|f| {
                f.extension_name_as_c_str()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<String>>()
    };

    debug!(
        "Loaded {} instance extension properties: {extension_properties:#?}",
        extension_properties.len()
    );
    let validation_layers = unsafe {
        entry
            .enumerate_instance_layer_properties()
            .unwrap()
            .iter()
            .map(|layer| {
                layer
                    .layer_name_as_c_str()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<String>>()
    };

    let enabled_layer_support = check_validation_layers(validation_layers);
    if enabled_layer_support {
        required_extensions.push(EXT_DEBUG_UTILS_NAME.as_ptr());
    }
    let mut instance_create_info = InstanceCreateInfo::default()
        .application_info(&application_info)
        .flags(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR)
        .enabled_extension_names(&required_extensions);

    let mut debug_create_info = DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            DebugUtilsMessageSeverityFlagsEXT::WARNING
                | DebugUtilsMessageSeverityFlagsEXT::INFO
                | DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            DebugUtilsMessageTypeFlagsEXT::GENERAL
                | DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(debug_callback));

    if enabled_layer_support {
        instance_create_info = instance_create_info.push_next(&mut debug_create_info);
    }

    let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };

    Ok(instance)
}

fn check_validation_layers(validation_layers: Vec<String>) -> bool {
    let validation_layer_tbc = vec![String::from("VK_LAYER_KHRONOS_validation")];
    let mut count = 0;

    for layer in validation_layer_tbc.clone() {
        if validation_layers.contains(&layer) {
            count = count.add(1);
        }
    }

    count == validation_layer_tbc.len()
}

pub fn create_debugger(
    entry: &Entry,
    instance: &Instance,
) -> (debug_utils::Instance, DebugUtilsMessengerEXT) {
    let mut debug_create_info = DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            DebugUtilsMessageSeverityFlagsEXT::WARNING
                | DebugUtilsMessageSeverityFlagsEXT::INFO
                | DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            DebugUtilsMessageTypeFlagsEXT::GENERAL
                | DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(debug_callback));
    let debug_instance = debug_utils::Instance::new(entry, instance);
    let debugger = unsafe {
        debug_instance
            .create_debug_utils_messenger(&debug_create_info, None)
            .unwrap()
    };
    (debug_instance, debugger)
}

unsafe extern "system" fn debug_callback(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_type: DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
    user_data: *mut c_void,
) -> u32 {
    unsafe {
        let p_callback_data = *callback_data;
        let message_id_name = p_callback_data
            .message_id_name_as_c_str()
            .unwrap()
            .to_string_lossy();
        let message_id_number = p_callback_data.message_id_number;
        let message = p_callback_data
            .message_as_c_str()
            .unwrap()
            .to_string_lossy();

        match message_severity {
            DebugUtilsMessageSeverityFlagsEXT::WARNING => {
                warn!("{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n");
            }
            DebugUtilsMessageSeverityFlagsEXT::ERROR => {
                error!("{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n")
            }
            _ => {
                info!("{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n");
            }
            _ => {
                info!("{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n");
            }
        }
    }
    0
}
