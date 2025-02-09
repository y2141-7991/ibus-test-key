use std::collections::HashMap;
use crate::input_mode::{InputMode, INPUT_MODE_TELEX};
use ibus_sys::{core::to_gboolean, engine::{ibus_engine_register_properties, IBusEngine}, glib::{g_object_ref_sink, gchar, gpointer}, prop_list::{ibus_prop_list_append, ibus_prop_list_new, IBusPropList}, property::{ibus_property_new, ibus_property_set_sub_props, IBusPropState_PROP_STATE_CHECKED, IBusPropState_PROP_STATE_UNCHECKED, IBusPropType_PROP_TYPE_MENU, IBusPropType_PROP_TYPE_RADIO, IBusProperty}, text::{IBusText, StringExt}};


pub struct PropController {
    prop_list: *mut IBusPropList,
    input_mode_prop: *mut IBusProperty,
    prop_dict: HashMap<String, *mut IBusProperty>,
}

impl PropController {
    pub fn new() -> Self {
        let initial_input_mode = INPUT_MODE_TELEX;
        let (input_mode_prop, prop_list, prop_dict) = Self::init_props(initial_input_mode);
        PropController { prop_list, input_mode_prop, prop_dict }
    }
    pub fn do_focus_in(&self, engine: *mut IBusEngine) {
        unsafe {
            ibus_engine_register_properties(engine, self.prop_list);
        }
    }

    fn init_props(init_input_mode: InputMode) -> (
        *mut IBusProperty,
        *mut IBusPropList,
        HashMap<String, *mut IBusProperty>
    ) {
        unsafe {
            let prop_list = g_object_ref_sink(ibus_prop_list_new() as gpointer) as *mut IBusPropList;
            let input_mode_prop = g_object_ref_sink(ibus_property_new(
                "InputMode\0".as_ptr() as *const gchar,
                IBusPropType_PROP_TYPE_MENU,
                format!("Telex").to_ibus_text(),
                "\0".as_ptr() as *const gchar,
                "Switch input mode".to_ibus_text(),
                to_gboolean(true),
                to_gboolean(true),
                IBusPropState_PROP_STATE_UNCHECKED,
                std::ptr::null_mut() as *mut IBusPropList,
            ) as gpointer) as *mut IBusProperty;
            ibus_prop_list_append(prop_list, input_mode_prop);

            let props = g_object_ref_sink(ibus_prop_list_new() as gpointer) as *mut IBusPropList;
            let mut prop_map: HashMap<String, *mut IBusProperty> = HashMap::new();

            let prop = g_object_ref_sink(ibus_property_new(
                (INPUT_MODE_TELEX.prop_name.to_string() + "\0").as_ptr() as *const gchar,
                IBusPropType_PROP_TYPE_RADIO,
                format!("Telex").to_ibus_text(),
                "\0".as_ptr() as *const gchar,
                std::ptr::null_mut() as *mut IBusText,
                to_gboolean(true),
                to_gboolean(true),
                if INPUT_MODE_TELEX.mode == init_input_mode.mode { IBusPropState_PROP_STATE_CHECKED } else { IBusPropState_PROP_STATE_UNCHECKED },
                std::ptr::null_mut() as *mut IBusPropList) as gpointer
            ) as *mut IBusProperty;
            prop_map.insert(INPUT_MODE_TELEX.prop_name.to_string(), prop);
            ibus_prop_list_append(props, prop);
            ibus_property_set_sub_props(input_mode_prop, props);
            
            (input_mode_prop, prop_list, prop_map)
        }
    }
}
