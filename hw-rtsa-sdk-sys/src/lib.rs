#[allow(dead_case)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod bindings {
	use handler_derive::FFICallbacks;
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}