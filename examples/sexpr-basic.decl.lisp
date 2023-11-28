(avatar "sleeping-flower"
    (parameters
        (bool "SF_Outer")
        (bool "GW_Slipper")
    )

    (animations
        (switch "アウター"
            (parameter "SF_Outer")
            (object "SF_Outer" :disabled false :enabled true)
        )

        (switch "スリッパ"
            (parameter "GW_Slipper")
            (object "GW_Slipper" :disabled false :enabled true)
        )
    )

    (menu
        (submenu "服"
            (toggle "アウター" :switch "SF_Outer")
            (toggle "スリッパ" :switch "GW_Slipper")
        )
    )
)
