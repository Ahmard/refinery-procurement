use foxtive_macros::generate_diesel_enum;

generate_diesel_enum!(CatalogCategory {
    Gasket,
    Valve,
    Pump,
    Instrumentation,
    HeatExchanger,
    HandTool,
    Other,
});
