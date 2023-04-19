# アバター定義ファイル
Declavatar のアバター定義ファイルは [KDL](https://kdl.dev) で記述する。基本的な構文についてはそちらを参照のこと。

## 構文
```kdl
version "1.0.0"

avatar {
    parameters {

    }

    animations {

    }

    drivers {

    }

    menu {

    }
}
```

## `version` ノード

* 定義ファイルのバージョンを指定する。
* 現在は `1.0.0` 固定。

## `avatar` ノード

* アバター定義の各種要素を入れる。
* 直下に定義可能なノードは次のとおり。
    - [parameters](./avatar-parameters.md)
    - [animations](./avatar-animations.md)
    - [drivers](./avatar-drivers.md)
    - [menu](./avatar-menu.md)
