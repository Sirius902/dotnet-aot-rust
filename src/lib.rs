use std::ffi::c_int;

extern "C" {
    #[cfg(feature = "static")]
    #[link_name = "RhInitialize"]
    fn rh_initialize() -> bool;

    fn dotnet_lib_add(a: c_int, b: c_int) -> c_int;
}

pub fn init() {
    #[cfg(feature = "static")]
    unsafe {
        assert!(rh_initialize(), "Failed to initialize .NET runtime");
    }
}

pub fn add(a: c_int, b: c_int) -> c_int {
    unsafe { dotnet_lib_add(a, b) }
}
