extern crate alloc;

pub mod bindings {
    hyperlight_component_macro::host_bindgen!();
}

pub mod resource;
pub mod types;
pub mod worker;

pub use resource::Resource;
