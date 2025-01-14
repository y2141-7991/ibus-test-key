use crate::attribute::IBusAttribute;

extern "C" {
    pub fn ibus_attr_list_new() -> *mut IBusAttrList;
    pub fn ibus_attr_list_append(attr_list: *mut IBusAttrList, attr: *mut IBusAttribute);
}

pub type IBusAttrList = [u64; 7usize];
