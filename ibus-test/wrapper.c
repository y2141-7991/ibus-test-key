#include <ibus.h>
#include <stdio.h>


#define IBUS_TYPE_MY_ENGINE (ibus_my_engine_get_type())

typedef gboolean (*ibus_my_engine_callback_key_event)(void* ctx, IBusEngine* engine, guint keyval, guint keycode, guint modifiers);

void ibus_my_engine_set_callback(void* ctx, ibus_my_engine_callback_key_event* cb);



static void* global_context = NULL;
static ibus_my_engine_callback_key_event global_key_event_cb = NULL;

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

G_DEFINE_TYPE(IbusMyEngine, ibus_my_engine, IBUS_TYPE_ENGINE)

static void ibus_my_engine_class_init(IbusMyEngineClass *klass) {
    IBusEngineClass *engine_class = IBUS_ENGINE_CLASS(klass);
    engine_class->process_key_event = ibus_my_engine_process_key_event;
}

static void ibus_my_engine_init(IbusMyEngine *engine) {
}

static gboolean ibus_my_engine_process_key_event(IBusEngine *engine, guint keyval, guint keycode, guint modifiers) {
    g_print("Key Pressed: keyval=%u, keycode=%u, modifiers=%u\n", keyval, keycode, modifiers);
    return global_key_event_cb(global_context, engine, keyval, keycode, modifiers);
}

void ibus_my_engine_set_callback(
    void* context,
    ibus_my_engine_callback_key_event* key_event_cb
) {
    printf(key_event_cb);
    global_context = context;
    global_key_event_cb = key_event_cb;
}

void ibus_main_init() {
    IBusBus *bus;
    IBusFactory *factory;

    ibus_init();
    printf("Size : %zu\n", IBUS_TYPE_MY_ENGINE);
    bus = ibus_bus_new();
    g_object_ref_sink(bus);

    g_signal_connect(bus, "disconnected", G_CALLBACK(ibus_quit), NULL);

    factory = ibus_factory_new(ibus_bus_get_connection(bus));
    g_object_ref_sink(factory);

    ibus_factory_add_engine(factory, "my-engine", IBUS_TYPE_MY_ENGINE);
    ibus_bus_request_name(bus, "org.freedesktop.IBus.MyEngine", 0);    

    // g_object_unref(factory);
    // g_object_unref(bus);
}
