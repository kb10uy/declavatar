local da = require("declavatar");

return da.avatar("avatar-name", {
    da.parameters({
        da.bool("bool-param", { save = true }),
    }),

    da.assets({
        da.animation("animation"),
    }),

    da.menu({
        da.button("hoge", da.drive_group("group-name", "option-name")),
    }),

    da.fx_controller({
        da.group_layer("layer-name", {
            driven_by = "param-name",
            da.group_option({}),
            da.group_option({}),
        }),
    }),
});
