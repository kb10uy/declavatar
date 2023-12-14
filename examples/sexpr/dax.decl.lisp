(use da :self)
(use dax :self)

(da/avatar "dax"
    (da/parameters
        (da/int "dax-parameter" :default dax/foo)
    )
)
