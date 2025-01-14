use crate::core::IBusBus;
use gio::DBusConnection;

extern "C" {
    pub fn ibus_factory_new(connection: *mut DBusConnection) -> *mut IBusFactory;
    pub fn ibus_bus_get_connection(bus: *mut IBusBus) -> *mut DBusConnection;
}

pub type IBusFactory = [u8; 11];
