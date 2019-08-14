use crate::ffi::{Request, Response};

pub type Handler = unsafe extern "C" fn(request: *mut Request, response: *mut Response);
pub type ContentProviderResourceReleaser = unsafe extern "C" fn();
