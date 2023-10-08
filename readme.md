# パスワードジェネレータ
Rust入門用のリポジトリ。Rustの文法やできること・できないことを色々試しながら模索できる場所として作成。

# Make

```makefile
# default: length=12 algorithm=simple
# algorithm => simple・numeric
make run length=10 algorithm=simple
```


## アルゴリズム

### Simple
大文字・小文字のアルファベットと数字、その他記号からなるランダムな文字列を生成。

### Numeric
数字のみのランダムな文字列を生成。