(use da :self)

(da/avatar "all-features"
    ; parameter definition
    (da/parameters
        (da/bool "BoolParam" :scope 'local)
        (da/int "IntParam" :default 42)
        (da/float "FloatParam" :save false)
    )

    ; asset definition
    (da/assets
        (da/animation "AnimationClip")
        (da/material "Material")
    )

    ; menu definition
    (da/menu
        (da/button "hoge" '(da/select-option "group-name" "option-name"))
        (da/toggle "fuga" '(da/drive-switch "switch-name"))
        (da/radial "piyo" '(da/drive-puppet "puppet-name"))
        #|
        (da/submenu "foo"
            (da/two-axis "bar"
                :horizontal '(da/drive-puppet "horizontal")
                :vertical '(da/drive-puppet "vertical")
            )
            (da/two-axis "baz"
                :up '(da/drive-puppet "up")
                :down '(da/drive-puppet "down")
                :left '(da/drive-puppet "left")
                :right '(da/drive-puppet "right")
            )
        )
        |#
    )
    #|
    (da/fx-controller
        (da/group-layer "表情右手"
            (da/driven-by "GestureRight")
            (da/default-mesh "Face")
            (da/option "handopen" :on-param 2
                (da/set-shape "eyebrow_笑顔" :value 1.0)
                (da/set-shape "eye_にっこり" :value 1.0)
                (da/set-shape "mouth_にっこり" :value 1.0)
            )
        )
    )
    |#
)
