#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]


use std::ffi::{c_void, CString};
use anyhow::Ok;
use ibus_sys::engine::IBusEngine;
use ibus_sys::core::ibus_main;
use ibus_sys::glib::guint;
use ibus_sys::keys::ibus_keyval_from_name;
use std::collections::HashMap;



#[derive(Hash, PartialEq)]
struct IBusKeyPattern {
    keyval: u32,
    modifier: u32
}

impl Eq for IBusKeyPattern {
    
}

impl IBusKeyPattern {
    fn new(keyval: u32, modifier: u32) -> Self {
        IBusKeyPattern {
            keyval,
            modifier,
        }
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


        Ok(IBusKeyMap {keymap: mapping})
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
    // keymap: IBusKy
    command_map: HashMap<&'static str, MyIBusEngineCommand>
}

impl MyIBusContext {
    fn new() -> Self {
        MyIBusContext {command_map: ibus_my_engine_command_map() }
    }
    fn process_key_event(&mut self, engine: *mut IBusEngine, keyval: guint, keycode: guint, modifiers: guint) {
        println!("keyval={keyval}, keycode={keycode}, modifiers={modifiers}");
    }

    fn run_event_listener(&mut self, engine: *mut IBusEngine) {
        
    }
}

pub(crate) type ibus_my_engine_callback_key_event = unsafe extern "C" fn(
    context: *mut c_void,
    engine: *mut IBusEngine,
    keyval: guint,
    keycode: guint,
    modifiers: guint,
) -> bool;

extern "C" {
    fn ibus_main_init();

    pub (crate) fn ibus_my_engine_set_callback(
        context: *mut c_void,
        key_event_callback: ibus_my_engine_callback_key_event,
    ) -> bool;
}

unsafe extern "C" fn process_key_event(
    context: *mut c_void,
    engine: *mut IBusEngine,
    keycode: guint,
    keyval: guint,
    modifiers: guint
) -> bool {
    println!("hi");
    let context = &mut *(context as *mut MyIBusContext);
    context.process_key_event(engine, keyval, keycode, modifiers);
    true
}

fn main() {
    let mut context = MyIBusContext::new();
    unsafe {
        // let ctx: *mut c_void = &mut context as ;
        // println!("{:?}", context);
        ibus_my_engine_set_callback(&mut context as *mut _ as *mut c_void, process_key_event);
        ibus_main_init();
        ibus_main();
    }
    println!("Hello, world!");
}



// struct MyContext {
//     some_data: i32,
// }

// extern "C" fn process_key_event_with_context(
//     ctx: *mut c_void,
//     _engine: *mut c_void,
//     keyval: guint,
//     keycode: guint,
//     modifiers: guint,
// ) -> guint {
//     let context = unsafe { &*(ctx as *mut MyContext) };

//     println!(
//         "Key event: keyval={}, keycode={}, modifiers={}, context data={}",
//         keyval, keycode, modifiers, context.some_data
//     );

//     0 // Not handled
// }

// type IBusCallbackKeyEvent = extern "C" fn(
//     ctx: *mut c_void,
//     engine: *mut c_void, // Replace with a proper `IBusEngine` type if available
//     keyval: guint,
//     keycode: guint,
//     modifiers: guint,
// ) -> guint;

// extern "C" fn process_key_event(
//     ctx: *mut c_void,
//     engine: *mut c_void,
//     keyval: guint,
//     keycode: guint,
//     modifiers: guint,
// ) -> guint {
//     println!(
//         "Key event: ctx={:?}, keyval={}, keycode={}, modifiers={}",
//         ctx, keyval, keycode, modifiers
//     );

//     // Process the event and return whether it was handled (1 for true, 0 for false)
//     if keyval == 65 { // Example: Check if the keyval is 'A'
//         1 // Handled
//     } else {
//         0 // Not handled
//     }
// }

// extern "C" {
//     fn ibus_engine_set_key_event_handler(
//         engine: *mut c_void,
//         handler: IBusCallbackKeyEvent,
//         user_data: *mut c_void,
//     );
// }


// fn main() {
//     let mut context = MyContext { some_data: 42 };
//     let engine: *mut c_void = std::ptr::null_mut();

//     unsafe {
//         ibus_engine_set_key_event_handler(
//             engine,
//             process_key_event_with_context,
//             &mut context as *mut _ as *mut c_void,
//         );
//     }

//     println!("Callback with context registered!");
// }
