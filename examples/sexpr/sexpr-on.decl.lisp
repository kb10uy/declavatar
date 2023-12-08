(use da :self)
(da/avatar "on-block"
    (da/parameters
        (da/vrc-builtin 'gesture)
    )

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
)
