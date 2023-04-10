# `animations` ノード

* Unity における Animator Controller の要素を定義する。
* `avatar` ノード配下に複数定義可能で、それらの子が記述順に処理される。

```kdl
animations {
    group "Foo" {

    }

    switch "Bar" {

    }

    puppet "Baz" {

    }
}
```

## `group` 択一グループ

```kdl
group "GroupName" {
    parameter "TargetParameter"
    parameter "Expression"
    prevent mouth=true eyelids=true
}
```

## `switch` 真偽値スイッチ

## `puppet` 無段階調整アニメーション
