# `parameters` ブロック

## `int/float/bool` パラメーター定義
AnimatorController, ExMenu/Param で使用するパラメーターを定義する。

### 定義可能位置
`parameter` ブロック直下。

### 構文
```kdl
int "parameter-name" ...
float "parameter-name" ...
bool "parameter-name" ...
```

### 引数・オプション
* **parameter-name**
    - パラメーター名。
* default=*any* (0 / 0.0 / false)
    - デフォルト値を指定する。
    - scope が `internal` の場合は指定できない。
* save=*boolean* (false)
    - アバター切り替えをまたいで保存するかどうかを指定する。
    - scope が `internal` の場合は指定できない。
* scope=*visibility* (`synced`)
    - パラメーターの同期属性を指定する。
    - 指定可能な値は以下のうちいずれか 1 つ(string)。
    - `synced` : リモートに同期する。
    - `local` : リモートに同期しないが ExParam には記録する。
    - `internal` : AnimatorController 内にのみ記録する。

