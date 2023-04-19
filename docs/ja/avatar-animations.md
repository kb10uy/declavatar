# `animations` ブロック

## `group` 択一グループ定義
int 値で定義するグループ内から一つ選ぶアニメーションを定義する。

### 構文
```kdl
group "group-name" {
    ...
}
```

### ブロック内要素
* **parameter** (1)
* mesh (0-1)
* prevent (0-1)
* default (0-1)
* **option** (1-)

## `switch` スイッチ定義
bool 値で制御するスイッチアニメーションを定義する。

### 構文
```kdl
group "group-name" {
    ...
}
```

### ブロック内要素
* **parameter** (1)
* mesh (0-1)
* prevent (0-1)
* shape (0-)
* object (0-)

## `puppet` スライダー定義
float 値で制御するなめらかに変化するアニメーションを定義する。

### 構文
```kdl
group "group-name" {
    ...
}
```

### ブロック内要素
* **parameter** (1)
* mesh (0-1)
* **keyframe** (1-)

## `param` 対象パラメーター指定
`group`, `switch`, `puppet` で参照するパラメーターを指定する。

### 構文
```kdl
param "param-name"
```

### 引数
* **param-name**
    - パラメーター名。
    - それぞれのアニメーション定義に対して適切な型で宣言されていない場合はコンパイルエラーが発生する。

## `mesh` 対象 MeshRenderer 指定
`group`, `switch`, `puppet` で操作するデフォルトの MeshRenderer を指定する。

### 構文
```kdl
mesh "mesh-name"
```

* **mesh-name**
    - メッシュ名。
    - スラッシュ記法で孫以降のオブジェクトを指定可能。

## `prevent` TrackingControl 指定
アニメーションがアクティブ時に TrackingControl でトラッキングを上書きするかどうかを指定する。

### 構文
```kdl
param "parameter-name"
```

### 引数
* eyelids=*bool* (false)
    - まぶたのトラッキングを無効にするかどうか。
* mouth=*bool* (false)
    - 口のトラッキングを無効にするかどうか。


## `default` デフォルト状態定義
各操作要素のデフォルト状態を定義する。

### 構文
```kdl
default {
    ...
}
```

### ブロック内要素
* shape (0-)
* object (0-)

## `option` オプション定義
`group` のオプションを定義する。

### 構文
```
// 1. compact form
// ラベル名でシェイプキーを直接指定する。
option "shape-name"

// 2. simple form
// 1 つだけの操作要素を指定する。
option "option-label" ...

// 3. full form
// 複数の操作要素を指定する。
option "option-label" {
    ...
}
```

### 引数 (compact form)
* **shape-name**
    - シェイプキー名。
    - このオプションのラベルとしても使用される。

### 引数 (simple form)
* **option-label**
    - オプションのラベル。
* shape=*string*
    - シェイプキー名。
* object=*string*
    - オブジェクト名。
* value=*any* (varies)
    - 後述の `shape`, `object` の同引数に準ずる。
* disabled=*any* / enabled=*any*
    - 後述の `shape`, `object` の同引数に準ずる。

### 引数 (full form)
* **option-label**
    - オプションのラベル。

### ブロック内要素 (full form)
* shape (0-)
* object (0-)

## `keyframe` キーフレーム定義
`puppet` のキーフレームを定義する。

### 構文
```kdl
keyframe 0.0 {
    ...
}
```

### 引数 (full form)
* **time**
    - キーフレームの位置。
    - 範囲は 0.0 - 1.0 であり、 Unity の Animation アセットの 0 - 100 フレームにマッピングされる。

### ブロック内要素 (full form)
* shape (0-)
* object (0-)

## `shape` シェイプキー操作
シェイプキーを操作する。

### 構文
```kdl
shape "shape-name" ...
```

### 引数
* **shape-name**
    - シェイプキー名。
* value=*float* (0.0 or 1.0)
    - シェイプキー移動量。
    - **`group` 内のみで有効。**
    - 範囲は 0.0 - 1.0 であり、 Unity の 0.0 - 100.0 にマッピングされる。
    - デフォルト値は `option` 内で定義されている場合は 1.0 に、`default` 内で定義されている場合は 0.0 になる。
* disabled=*float* / enabled=*float*
    - シェイプキー移動量。
    - **`switch` 内でのみ有効。**
    - それぞれ無効時と有効時の値を指定する。
* mesh=*string*
    - 操作対象の MeshRenderer 名を上書きする。
    - 定義されているブロックで `mesh` が指定されている場合、デフォルト値はそのメッシュになる。
    - `mesh` が指定されていない場合、省略するとコンパイルエラーが発生する。

## `object` オブジェクト切り替え
オブジェクトの有効・無効を切り替える。

### 構文
```kdl
shape "object-name" ...
```

### 引数
* **object-name**
    - オブジェクト名。
* value=*bool* (false or true)
    - アクティブ状態。
    - **`group` 内のみで有効。**
    - デフォルト値は `option` 内で定義されている場合は true に、`default` 内で定義されている場合は false になる。
* disabled=*bool* / enabled=*bool*
    - アクティブ状態。
    - **`switch` 内でのみ有効。**
    - それぞれ無効時と有効時の値を指定する。
