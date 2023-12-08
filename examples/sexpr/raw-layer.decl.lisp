(use da :self)

(da/avatar "raw-layer"
    ; parameter definition
    (da/parameters
        (da/int "hoge")
        (da/bool "fuga")
        (da/int "piyo")
        (da/float "state1-speed" :scope 'internal)
        (da/float "BlendX" :scope 'internal)
        (da/float "BlendY" :scope 'internal)
    )

    ; FX controller definition
    (da/fx-controller
        (da/raw-layer "raw"
            :default "state1"
            (da/state "state1"
                (da/clip "animation-clip" :speed 1.0 :speed-by "state1-speed")
                (da/transition-to "state2" :duration 0.0 '(= "hoge" 0) '(= "fuga" true))
                (da/transition-to "state2" :duration 0.5 '(> "piyo" 1))
            )
            (da/state "state2"
                (da/blendtree :type '2d-cartesian :x "BlendX" :y "BlendY"
                    (da/blend-field "neutral" 0.0 0.0)
                    (da/blend-field "right" 1.0 0.0)
                    (da/blend-field "left" -1.0 0.0)
                    (da/blend-field "up" 0.0 1.0)
                    (da/blend-field "down" 0.0 -1.0)
                )
                (da/transition-to "state1" '(!= "hoge" 0))
            )
        )
    )
)
