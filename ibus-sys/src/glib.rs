use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type GCallBack = extern "C" fn() -> ();

extern "C" {
    pub fn g_object_ref_sink(object: gpointer) -> gpointer;
    pub fn ibus_object_get_type() -> GType;
    pub fn ibus_engine_get_type() -> GType;
}


pub type gchar = c_char;
pub type guint = c_uint;
pub type gboolean = c_int;
pub type gsize = c_ulong;
pub type gssize = c_long;
pub type gint = c_int;
pub type gpointer = *mut c_void;

pub type GType = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GArray {
    pub data: *mut c_char,
    pub len: guint,
}


