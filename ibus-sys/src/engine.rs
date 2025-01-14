use crate::glib::{gboolean, guint};
use crate::lookup_table::IBusLookupTable;
use crate::prop_list::IBusPropList;
use crate::property::IBusProperty;
use crate::text::IBusText;

extern "C" {
    pub fn ibus_engine_commit_text(engine: *mut IBusEngine, text: *mut IBusText);
    pub fn ibus_engine_hide_lookup_table(engine: *mut IBusEngine);
    pub fn ibus_engine_update_preedit_text(
        engine: *mut IBusEngine,
        text: *mut IBusText,
        cursor_pos: guint,
        visible: gboolean,
    );
    pub fn ibus_engine_hide_preedit_text(engine: *mut IBusEngine);
    pub fn ibus_engine_hide_auxiliary_text(engine: *mut IBusEngine);
    pub fn ibus_engine_update_auxiliary_text(
        engine: *mut IBusEngine,
        text: *mut IBusText,
        visible: gboolean,
    );
    pub fn ibus_engine_update_lookup_table(
        engine: *mut IBusEngine,
        lookup_table: *mut IBusLookupTable,
        visible: gboolean,
    );
    pub fn ibus_engine_register_properties(engine: *mut IBusEngine, prop_list: *mut IBusPropList);
    pub fn ibus_engine_update_property(engine: *mut IBusEngine, prop: *mut IBusProperty);

}

pub type IBusEngine = [u64; 11usize];
