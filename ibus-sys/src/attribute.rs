use crate::glib::guint;
use std::os::raw::c_uint;

pub const IBusAttrType_IBUS_ATTR_TYPE_UNDERLINE: IBusAttrType = 1;
pub const IBusAttrType_IBUS_ATTR_TYPE_FOREGROUND: IBusAttrType = 2;
pub const IBusAttrType_IBUS_ATTR_TYPE_BACKGROUND: IBusAttrType = 3;

pub type IBusAttrType = c_uint;

pub const IBusAttrUnderline_IBUS_ATTR_TYPE_NONE: IBusAttrUnderline = 0;
pub const IBusAttrUnderline_IBUS_ATTR_TYPE_SINGLE: IBusAttrUnderline = 1;
pub const IBusAttrUnderline_IBUS_ATTR_TYPE_DOUBLE: IBusAttrUnderline = 2;
pub const IBusAttrUnderline_IBUS_ATTR_TYPE_LOW: IBusAttrUnderline = 3;
pub const IBusAttrUnderline_IBUS_ATTR_TYPE_ERROR: IBusAttrUnderline = 4;

pub type IBusAttrUnderline = c_uint;
pub type IBusAttribute = [u64; 8usize];

extern "C" {
    pub fn ibus_attribute_new(
        type_: guint,
        value: guint,
        start_index: guint,
        end_index: guint,
    ) -> *mut IBusAttribute;
}
