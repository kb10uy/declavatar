(da/fx-controller
    (da/group-layer "服装" :driven-by "Clothes"
        (da/option 'default
            (da/set-object "Stockings" :value true)
            (da/set-object "Shoes" :value true)
            (da/set-object "Jacket" :value true)
        )
        (da/option "裸足"
            (da/set-object "Stockings" :value false)
            (da/set-object "Shoes" :value false)
        )
    )

    (da/switch-layer "帽子" :driven-by "Cap"
        (da/option 'disabled (da/set-object "Cap" :value false))
        (da/option 'enabled (da/set-object "Cap" :value true))
    )

    (da/puppet-layer "ロングもみあげ" :driven-by "Momiage" :default-mesh "Hair"
        (da/option 0.0
            (da/set-shape "もみあげ" :value 0.0)
            (da/set-shape "もみあげ2" :value 0.0)
        )

        (da/option 0.5
            (da/set-shape "もみあげ" :value 1.0)
            (da/set-shape "もみあげ2" :value 0.0)
        )

        (da/option 1.0
            (da/set-shape "もみあげ" :value 1.0)
            (da/set-shape "もみあげ2" :value 1.0)
        )
    )
)
