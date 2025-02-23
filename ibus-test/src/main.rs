#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use anyhow::Ok;
use ibus_sys::attr_list::{ibus_attr_list_append, ibus_attr_list_new};
use ibus_sys::attribute::{
    ibus_attribute_new, IBusAttrType_IBUS_ATTR_TYPE_BACKGROUND,
    IBusAttrType_IBUS_ATTR_TYPE_FOREGROUND, IBusAttrType_IBUS_ATTR_TYPE_UNDERLINE,
    IBusAttrUnderline_IBUS_ATTR_TYPE_SINGLE,
};
use ibus_sys::core::{
    ibus_main, to_gboolean, IBusModifierType_IBUS_CONTROL_MASK, IBusModifierType_IBUS_MOD1_MASK,
    IBusModifierType_IBUS_RELEASE_MASK,
};
use ibus_sys::engine::{
    self, ibus_engine_commit_text, ibus_engine_hide_lookup_table, ibus_engine_update_lookup_table,
    ibus_engine_update_preedit_text, IBusEngine,
};
use ibus_sys::glib::{gchar, gint, guint};
use ibus_sys::ibus_keysyms::{
    IBUS_KEY_BackSpace, IBUS_KEY_Down, IBUS_KEY_Left, IBUS_KEY_Return, IBUS_KEY_Right, IBUS_KEY_Up,
    IBUS_KEY_a, IBUS_KEY_s, IBUS_KEY_space, IBUS_KEY_z, IBUS_KEY_A, IBUS_KEY_Z,
};
use ibus_sys::keys::ibus_keyval_from_name;
use ibus_sys::lookup_table::IBusLookupTable;
use ibus_sys::text::{ibus_text_set_attributes, StringExt};
use prop_controller::PropController;
use std::collections::HashMap;
use std::ffi::{c_void, CString};

mod input_mode;
mod prop_controller;

#[derive(Hash, PartialEq)]
struct IBusKeyPattern {
    keyval: u32,
    modifier: u32,
}

impl Eq for IBusKeyPattern {}

impl IBusKeyPattern {
    fn new(keyval: u32, modifier: u32) -> Self {
        IBusKeyPattern { keyval, modifier }
    }
}

struct IBusKeyMap {
    keymap: HashMap<IBusKeyPattern, String>,
}

impl IBusKeyMap {
    fn to_ibus_key(s: &str) -> guint {
        let cs = CString::new(s.to_string()).unwrap();
        unsafe { ibus_keyval_from_name(cs.as_ptr()) }
    }
    fn new(keymap: HashMap<IBusKeyPattern, String>) -> anyhow::Result<Self> {
        let mut mapping: HashMap<IBusKeyPattern, String> = HashMap::new();

        Ok(IBusKeyMap { keymap: mapping })
    }
}

type MyIBusEngineCommand = fn(&mut MyIBusContext, *mut IBusEngine) -> bool;

pub(crate) fn ibus_my_engine_command_map() -> HashMap<&'static str, MyIBusEngineCommand> {
    let mut mapping: HashMap<&str, MyIBusEngineCommand> = HashMap::new();

    let mut register = |name: &'static str, cmd: MyIBusEngineCommand| mapping.insert(name, cmd);
    register("escape", |context, engine| {
        println!("{:?}", engine);
        true
    });
    mapping
}

#[repr(C)]
struct MyIBusContext {
    command_map: HashMap<&'static str, MyIBusEngineCommand>,
    prop_controller: PropController,
    raw_input: String,
    preedit: String,
    auxiliary_text: String,
    cursor_pos: usize,
    lookup_table: IBusLookupTable,
}

impl MyIBusContext {
    fn new() -> Self {
        MyIBusContext {
            command_map: ibus_my_engine_command_map(),
            prop_controller: PropController::new(),
            raw_input: String::new(),
            preedit: String::new(),
            auxiliary_text: String::new(),
            cursor_pos: 0,
            lookup_table: IBusLookupTable::new(10, 0, 1, 1),
        }
    }
    fn process_key_event(
        &mut self,
        engine: *mut IBusEngine,
        keyval: guint,
        keycode: guint,
        modifiers: guint,
    ) -> bool {
        println!("Process key event: keyval={keyval}, keycode={keycode}, modifiers={modifiers}");

        if modifiers & IBusModifierType_IBUS_RELEASE_MASK != 0 {
            return false;
        }

        if modifiers == IBusModifierType_IBUS_CONTROL_MASK && keyval == IBUS_KEY_s {
            self.ibus_my_engine_update_lookup_table(engine);
            return true;
        }

        if modifiers != 0 {
            if self.preedit.len() == 0 {
                return false;
            } else {
                return true;
            }
        }

        if !self.preedit.is_empty() {
            if keyval == IBUS_KEY_space {
                self.preedit.push(' ');
                return self.ibus_my_engine_commit_preedit_string(engine);
            }
            if keyval == IBUS_KEY_Return {
                return self.ibus_my_engine_commit_preedit_string(engine);
            }

            if keyval == IBUS_KEY_BackSpace {
                if self.preedit.len() == 0 {
                    return false;
                }
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                    self.preedit.remove(self.cursor_pos);
                    self.ibus_my_engine_update(engine);
                }
            }
            if keyval == IBUS_KEY_Left {}
            if keyval == IBUS_KEY_Right {}
            if keyval == IBUS_KEY_Up {}
            if keyval == IBUS_KEY_Down {}
        }

        if (IBUS_KEY_A..IBUS_KEY_Z + 1).contains(&keyval)
            || (IBUS_KEY_a..IBUS_KEY_z + 1).contains(&keyval)
        {
            if modifiers & (IBusModifierType_IBUS_CONTROL_MASK | IBusModifierType_IBUS_MOD1_MASK)
                == 0
            {
                let _text = char::from_u32(keyval).unwrap();
                self.preedit.insert(self.cursor_pos, _text);
                self.cursor_pos += 1;
                println!("{}, {}", self.preedit, self.cursor_pos);
                self.ibus_my_engine_update(engine);
                return true;
            }
        } else {
            if keyval < 128 {}
        }

        false
    }

    fn ibus_my_engine_commit_preedit_string(&mut self, engine: *mut IBusEngine) -> bool {
        if self.preedit.len() == 0 {
            return false;
        }
        unsafe {
            ibus_engine_commit_text(engine, self.preedit.to_ibus_text());
        }
        self.preedit = String::new();
        self.cursor_pos = 0;
        self.ibus_my_engine_update(engine);
        true
    }

    fn ibus_my_engine_update(&mut self, engine: *mut IBusEngine) {
        self.ibus_my_engine_update_preedit(engine);
        // self.ibus_my_engine_update_auxiliary_text(engine);
        // self.ibus_my_engine_update_lookup_table(engine);
        unsafe {
            ibus_engine_hide_lookup_table(engine);
        }
    }

    fn ibus_my_engine_update_preedit(&mut self, engine: *mut IBusEngine) {
        unsafe {
            let preedit_attrs = ibus_attr_list_new();
            ibus_attr_list_append(
                preedit_attrs,
                ibus_attribute_new(
                    IBusAttrType_IBUS_ATTR_TYPE_BACKGROUND,
                    IBusAttrUnderline_IBUS_ATTR_TYPE_SINGLE,
                    0,
                    self.preedit.len() as guint,
                ),
            );
            let preedit_text = self.preedit.to_ibus_text();
            ibus_text_set_attributes(preedit_text, preedit_attrs);
            ibus_engine_update_preedit_text(
                engine,
                preedit_text,
                self.cursor_pos as u32,
                to_gboolean(true),
            );
        }
    }

    fn ibus_my_engine_update_auxiliary_text(&mut self, engine: *mut IBusEngine) {}

    fn ibus_my_engine_update_lookup_table(&mut self, engine: *mut IBusEngine) {
        unsafe {
            let visible = self.lookup_table.get_number_candidates();
            ibus_engine_update_lookup_table(
                engine,
                &mut self.lookup_table as *mut IBusLookupTable,
                visible as i32,
            );
        }
    }

    fn ibus_my_engine_do_focus_in(&mut self, engine: *mut IBusEngine) {
        println!("Focus In");
        self.prop_controller.do_focus_in(engine);
    }

    fn ibus_my_engine_do_candidate_clicked(
        &mut self,
        engine: *mut IBusEngine,
        index: guint,
        button: guint,
        state: guint,
    ) -> bool {
        println!("do candidate clicked");
        let page_size = self.lookup_table.get_page_size();
        if index > page_size {
            return false;
        }
        let page = self.lookup_table.get_cursor_pos() / page_size;

        let new_pos = page * page_size + index;

        if new_pos >= self.lookup_table.get_number_candidates() {
            return false;
        }

        self.lookup_table.set_cursor_pos(new_pos);
        let cursor = self.lookup_table.get_cursor_pos();

        true
    }

    fn ibus_my_engine_commit_candidate(&mut self, engine: *mut IBusEngine) {
        self.preedit = String::from("");
        self.ibus_my_engine_commit_preedit_string(engine);
    }
}

pub(crate) type ibus_my_engine_callback_key_event = unsafe extern "C" fn(
    context: *mut c_void,
    engine: *mut IBusEngine,
    keyval: guint,
    keycode: guint,
    modifiers: guint,
) -> bool;

pub(crate) type ibus_my_engine_callback_candidate_clicked = unsafe extern "C" fn(
    context: *mut c_void,
    engine: *mut IBusEngine,
    index: guint,
    button: guint,
    state: guint,
) -> bool;

pub(crate) type ibus_my_engine_callback_focus_in =
    unsafe extern "C" fn(context: *mut c_void, engine: *mut IBusEngine);

pub(crate) type ibus_my_engine_callback_property_activate = unsafe extern "C" fn(
    context: *mut c_void,
    engine: *mut IBusEngine,
    prop_name: *mut gchar,
    prop_state: guint,
);

extern "C" {
    fn ibus_main_init();

    pub(crate) fn ibus_my_engine_set_callback(
        context: *mut c_void,
        key_event_callback: ibus_my_engine_callback_key_event,
        candidated_click_cb: ibus_my_engine_callback_candidate_clicked,
        focus_in_cb: ibus_my_engine_callback_focus_in,
        property_activate_cb: ibus_my_engine_callback_property_activate,
    );
}

unsafe extern "C" fn process_key_event(
    context: *mut c_void,
    engine: *mut IBusEngine,
    keyval: guint,
    keycode: guint,
    modifiers: guint,
) -> bool {
    let context = &mut *(context as *mut MyIBusContext);
    context.process_key_event(engine, keyval, keycode, modifiers);
    true
}

unsafe extern "C" fn candidated_clicked(
    context: *mut c_void,
    engine: *mut IBusEngine,
    index: guint,
    button: guint,
    state: guint,
) -> bool {
    println!("{}, {}, {}", index, button, state);
    true
}

unsafe extern "C" fn focus_in(context: *mut c_void, engine: *mut IBusEngine) {
    let context = &mut *(context as *mut MyIBusContext);
    context.ibus_my_engine_do_focus_in(engine);
}

unsafe extern "C" fn property_activate(
    context: *mut c_void,
    engine: *mut IBusEngine,
    prop_name: *mut gchar,
    prop_state: guint,
) {
    println!("{:?}, {}", prop_name, prop_state);
}

fn main() {
    let mut context = MyIBusContext::new();
    unsafe {
        ibus_my_engine_set_callback(
            &mut context as *mut _ as *mut c_void,
            process_key_event,
            candidated_clicked,
            focus_in,
            property_activate,
        );
        ibus_main_init();
        ibus_main();
    }
}
