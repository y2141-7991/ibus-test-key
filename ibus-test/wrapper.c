#include <ibus.h>
#include <stdio.h>

#define IBUS_TYPE_MY_ENGINE (ibus_my_engine_get_type())

typedef gboolean (*ibus_my_engine_callback_key_event)(void* ctx, IBusEngine* engine, guint keyval, guint keycode, guint modifiers);
typedef gboolean (*ibus_my_engine_callback_candidate_clicked)(void* ctx, IBusEngine* engine, guint index, guint button, guint state);
typedef void (*ibus_my_engine_callback_focus_in)(void* ctx, IBusEngine* engine);
typedef void (*ibus_my_engine_callback_property_activate)(void* ctx, IBusEngine* engine, const gchar prop_name, guint prop_state);

void ibus_my_engine_set_callback(void* ctx, ibus_my_engine_callback_key_event* cb, ibus_my_engine_callback_candidate_clicked*, ibus_my_engine_callback_focus_in*, ibus_my_engine_callback_property_activate*);



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
static gboolean ibus_my_engine_process_key_event(IBusEngine *engine, guint keyval, guint keycode, guint modifiers);

static void ibus_my_engine_focus_in(IBusEngine *engine);
static void ibus_my_engine_property_activate(IBusEngine *engine, const gchar *prop_name, guint prop_state);


G_DEFINE_TYPE(IbusMyEngine, ibus_my_engine, IBUS_TYPE_ENGINE)



static void ibus_my_engine_init(IbusMyEngine *engine) {
}

static gboolean ibus_my_engine_process_key_event(IBusEngine *engine, guint keyval, guint keycode, guint modifiers) {
    g_print("Key Pressed: keyval=%u, keycode=%u, modifiers=%u\n", keyval, keycode, modifiers);
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
    printf(focus_in_cb);
    global_context = context;
    global_key_event_cb = key_event_cb;
    global_candidate_clicked_cb = candidated_click_cb;
    global_focus_in_cb = focus_in_cb;
    global_property_activate_cb = property_activate_cb;
}

static gboolean ibus = FALSE;
#define PKGDATADIR "@DATADIR@/ibus-test-key/"

void ibus_main_init() {
    void* context;
    IBusBus *bus;
    IBusFactory *factory;

    ibus_init();
    printf("Size : %zu\n\n\n\n\n", IBUS_TYPE_MY_ENGINE);
    g_print(PKGDATADIR);
    bus = ibus_bus_new();
    g_object_ref_sink(bus);

    g_signal_connect(bus, "disconnected", G_CALLBACK(ibus_quit), NULL);

    factory = ibus_factory_new(ibus_bus_get_connection(bus));
    g_object_ref_sink(factory);

    ibus_factory_add_engine(factory, "my-engine", IBUS_TYPE_MY_ENGINE);
    if (ibus) {
        ibus_bus_request_name(bus, "org.freedesktop.IBus.MyEngine", 0); 
    } else {
        IBusComponent* component;
        component = ibus_component_new("org.freedesktop.IBus.MyEngine",
                                        "MyEngine",
                                        "0.1.0",
                                        "GPL",
                                        "Y <y.nguyen@gmail.com>",
                                        "",
                                        "",
                                        "ibus-test-key");
        ibus_component_add_engine (component,
                                   ibus_engine_desc_new ("my-engine",
                                                         "my-engine",
                                                         "my-engine",
                                                         "",
                                                         "GPL",
                                                         "Y <ndty14@gmail.com>",
                                                         PKGDATADIR"/icons/ibus-enchant.svg",
                                                         "vn"));
        ibus_bus_register_component (bus, component);
    }
     
  
    // ibus_my_engine_set_callback(context, ibus_my_engine_process_key_event);
    // g_object_unref(factory);
    // g_object_unref(bus);
}
