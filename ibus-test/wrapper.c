#include <ibus.h>
#include <stdio.h>
#include "wrapper.h"
#include "config.h"

#define IBUS_TYPE_MY_ENGINE (ibus_my_engine_get_type())

GType ibus_my_engine_get_type(void);



static void* global_context = NULL;
static ibus_my_engine_callback_key_event global_key_event_cb = NULL;
static ibus_my_engine_callback_candidate_clicked global_candidate_clicked_cb = NULL;
static ibus_my_engine_callback_focus_in global_focus_in_cb = NULL;
static ibus_my_engine_callback_property_activate global_property_activate_cb = NULL;


struct _IbusMyEngine {
    IBusEngine parent_instance;
};

struct _IbusMyEngineClass {
    IBusEngineClass parent_class;
};
typedef struct _IbusMyEngine IbusMyEngine;
typedef struct _IbusMyEngineClass IbusMyEngineClass;


static void ibus_my_engine_class_init(IbusMyEngineClass *klass);
static void ibus_my_engine_init(IbusMyEngine *engine);
static void ibus_my_engine_destroy(IbusMyEngine *engine);

static gboolean ibus_my_engine_process_key_event(IBusEngine *engine, guint keyval, guint keycode, guint modifiers);
static void ibus_my_engine_focus_in(IBusEngine *engine);
static void ibus_my_engine_property_activate(IBusEngine *engine, const gchar *prop_name, guint prop_state);


G_DEFINE_TYPE(IbusMyEngine, ibus_my_engine, IBUS_TYPE_ENGINE)


static void ibus_my_engine_init(IbusMyEngine *my_engine) {
}

static void ibus_my_engine_destroy(IbusMyEngine *engine) {
    ((IBusObjectClass *)ibus_my_engine_parent_class)->destroy((IBusObject *)engine);
}

static gboolean ibus_my_engine_process_key_event(IBusEngine *engine, guint keyval, guint keycode, guint modifiers) {
    return global_key_event_cb(global_context, engine, keyval, keycode, modifiers);
}

static gboolean ibus_my_engine_candidate_clicked(IBusEngine *engine, int index, int button, int state) {
    return global_candidate_clicked_cb(global_context, engine, index, button, state);
}

static void ibus_my_engine_focus_in(IBusEngine *engine) {
    global_focus_in_cb(global_context, engine);
}

static void ibus_my_engine_property_activate(IBusEngine *engine, const gchar *prop_name, guint prop_state) {
    global_property_activate_cb(global_context, engine, prop_name, prop_state);
}

static void ibus_my_engine_class_init(IbusMyEngineClass *klass) {
    IBusEngineClass *engine_class = IBUS_ENGINE_CLASS(klass);
    IBusObjectClass *object_class = IBUS_OBJECT_CLASS(klass);

    object_class->destroy = (IBusObjectDestroyFunc)ibus_my_engine_destroy;

    engine_class->process_key_event = ibus_my_engine_process_key_event;
    engine_class->candidate_clicked = ibus_my_engine_candidate_clicked;
    engine_class->focus_in = ibus_my_engine_focus_in;
    engine_class->property_activate = ibus_my_engine_property_activate;
}

void ibus_my_engine_set_callback(
    void* context,
    ibus_my_engine_callback_key_event* key_event_cb,
    ibus_my_engine_callback_candidate_clicked* candidated_click_cb,
    ibus_my_engine_callback_focus_in* focus_in_cb,
    ibus_my_engine_callback_property_activate* property_activate_cb
) {
    global_context = context;
    global_key_event_cb = key_event_cb;
    global_candidate_clicked_cb = candidated_click_cb;
    global_focus_in_cb = focus_in_cb;
    global_property_activate_cb = property_activate_cb;
}


static void ibus_disconnected_cb(IBusBus *bus, gpointer user_data) {
  ibus_quit();
}

static gboolean ibus = FALSE;

void ibus_main_init() {
    IBusBus *bus;
    IBusFactory *factory;

    ibus_init();

    bus = ibus_bus_new();
    g_object_ref_sink(bus);
    IBUS_Escape
    g_signal_connect(bus, "disconnected", G_CALLBACK(ibus_disconnected_cb), NULL);

    factory = ibus_factory_new(ibus_bus_get_connection(bus));
    g_object_ref_sink(factory);

    ibus_factory_add_engine(factory, "myengine-sample", IBUS_TYPE_MY_ENGINE);
    if (ibus) {
        ibus_bus_request_name(bus, "org.freedesktop.IBus.MyEngine", 0); 
    } else {
        IBusComponent* component;
        component = ibus_component_new("org.freedesktop.IBus.MyEngine",
                                        "MyEngine",
                                        "0.1",
                                        "GPL",
                                        "Y <y.nguyen@gmail.com>",
                                        "",
                                        "",
                                        "myengine-sample");
        ibus_component_add_engine (component,
                                   ibus_engine_desc_new ("myengine-sample",
                                                         "myengine-sample",
                                                         "myengine-sample",
                                                         "vn",
                                                         "GPL",
                                                         "Y <ndty14@gmail.com>",
                                                         PKGDATADIR "/iconnn.svg",
                                                         "default"));
        ibus_bus_register_component (bus, component);
        
    }
}
