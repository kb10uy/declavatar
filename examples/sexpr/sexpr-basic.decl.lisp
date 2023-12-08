(use da :self)

(da/avatar "all-features"
    ; parameter definition
    (da/parameters
        (da/int "Emote" :default 42)
        (da/bool "Hat" :scope 'local)
        (da/float "Eyelids" :save false)
    )

    ; FX controller definition
    (da/fx-controller
        (da/group-layer "表情"
            :driven-by "Emote"
            :default-mesh "Face"
            (da/option "smile"
                (da/set-shape "eyebrow_笑顔")
                (da/set-shape "eye_にっこり")
                (da/set-shape "mouth_にっこり")
            )
            (da/option "angry"
                (da/set-shape "eyebrow_真面目" :value 1.0)
                (da/set-shape "eye_むっ" :value 0.5)
                (da/set-shape "mouth_への字" :value 0.5)
            )
        )

        (da/switch-layer "帽子"
            :driven-by "Hat"
            (da/option 'disabled
                (da/set-object "Hat" :value false)
            )
            (da/option 'enabled
                (da/set-object "Hat" :value true)
            )
        )

        (da/puppet-layer "目閉じ"
            :driven-by "Eyelids"
            (da/option 0.0
                (da/set-shape "eye_まばたき" :value 0.0)
            )
            (da/option 1.0
                (da/set-shape "eye_まばたき" :value 1.0)
            )
        )
    )

    ; menu definition
    (da/menu
        (da/toggle "帽子オンオフ" (da/drive-switch "帽子"))
        (da/radial "まばたき" (da/drive-puppet "目閉じ"))
        (da/submenu "表情"
            (da/toggle "にっこり" (da/drive-group "表情" "smile"))
            (da/toggle "むっ" (da/drive-group "表情" "angry"))
        )
    )
)
