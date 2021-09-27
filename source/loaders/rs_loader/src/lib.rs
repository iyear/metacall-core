pub use std::ffi::c_void;
pub use std::os::raw::c_int;
pub use std::os::raw::c_char;

pub type PointerToCvoid = *mut c_void;
pub type Path = *const *mut c_char;

mod lifecycle;

pub use lifecycle::rs_loader_impl_initialize;
pub use lifecycle::rs_loader_impl_load_from_file;
pub use lifecycle::rs_loader_impl_load_from_memory;
pub use lifecycle::rs_loader_impl_load_from_package;
pub use lifecycle::rs_loader_impl_discover;
pub use lifecycle::rs_loader_impl_clear;
pub use lifecycle::rs_loader_impl_destroy;
