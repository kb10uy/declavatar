(use da :self)

; 下まつ毛を消しつつ、デフォルトの目のときだけジト目にしたいものとする。
(define (default-face-set enabled)
    `(
        ,(da/set-shape "eyelids_ジト目" :value (if enabled 0.5 0.0))
        ,(da/set-shape "eyelids_下まつ毛消し" :value 1.0)
    )
)

(da/avatar "on-block"
    (da/parameters
        (da/int "Eyelids")
    )

    (da/fx-controller
        (da/group-layer "まぶた"
            :driven-by "Eyelids"
            :default-mesh "Face"
            (apply da/option 'default
                (default-face-set true)
            )
            (apply da/option "笑い"
                (da/set-shape "eyelid_笑い" :value 1.0)
                (default-face-set false)
            )
        )
    )
)
