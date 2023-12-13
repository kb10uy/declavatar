(use da :self)

(define hyojo-defaults
    `(
        ,(da/set-shape "only-default")
        ,(da/set-shape "both" :value 0.2)
        ,(da/set-shape "only-option")
    )
)

(da/avatar "defaults"
    (da/parameters
        (da/int "emote")
    )

    (da/fx-controller
        (da/group-layer "hyojo" :driven-by "emote" :default-mesh "face"
            (apply da/option 'default `(
                ,@hyojo-defaults
            ))
            (apply da/option "option" `(
                ,@hyojo-defaults
                ,(da/set-shape "both")
                ,(da/set-shape "only-option" :value 0.8)
            ))
        )
    )
)
