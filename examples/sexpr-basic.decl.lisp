(declare-avatar "sleeping-flower"
    (declare-parameters
        (declare-bool "SF_Outer")
        (declare-bool "GW_Slipper")
    )

    (da/parameters
        (da/bool "SF_Outer")
        (da/bool "GW_Slipper")
    )

    (declare-assets
        ("")
    )

    (declare-animations
        (declare-switch "アウター"
            (use-parameter "SF_Outer")
            (switch-object "SF_Outer" :disabled false :enabled true)
        )

        (declare-switch "スリッパ"
            (use-parameter "GW_Slipper")
            (switch-object "GW_Slipper" :disabled false :enabled true)
        )
    )

    (declare-menu
        (declare-submenu "服"
            (declare-toggle "アウター" :switch "SF_Outer")
            (declare-toggle "スリッパ" :switch "GW_Slipper")
        )
    )
)
