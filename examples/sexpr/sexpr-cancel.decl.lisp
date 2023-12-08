(use da :self)

(define (default-face-set enabled)
    `(
        ,(da/set-shape "eyelids_ジト目" :value (if enabled 0.5 0.0))
    )
)

(da/avatar "on-block"
    (da/parameters
        (da/int "Eyelids")
    )

    (da/fx-controller
        (da/group-layer "まぶた"
            (da/driven-by "Eyelids")
            (da/default-mesh "Face")
            (da/default-option
                ,@(default-face-set true)
            )
            (da/option "笑い"
                (da/set-shape "eyelid_笑い" :value 1.0)
                ,@(default-face-set false)
            )
        )
    )
)
