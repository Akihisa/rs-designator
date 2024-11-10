# 参照名ライブラリ

参照名を個別にばらしたり、まとめて文字列にするためのライブラリ

ex)
- 個別: R1,R2,R3,R4,R10,R11...
- まとめ: R1-4,10,11

## 構成

以下のモジュールで構成する。

### designator

参照名文字列を `prefix`, `number`, `suffix` に分解して管理する。

3つのパートに分けることで、以下の機能を実現する。

- 自然な並び替え
- 参照名間の差分
- 参照名の省略形
- 括弧による分類

#### 自然な並び替え

文字列のままだと、並び替えたときに R1,R10,R2 というようになってしまう。
それを防ぐために、各パートごとで並び替えることができるようになる。

並び替えの優先度は、`prefix`, `number`, `suffix`

#### 参照名間の差分

`prefix` が同じであれば、`number` もしくは `suffix` により差分を取ることができる。

#### 参照名の省略形

R1,R2,R3,R4 みたいな文字列を R1,2,3,4 のように、可読性をあげるための処理をおこなう。

他の参照名を各パートごとに比較し、同じ場合は省略する。

#### 括弧による分類

括弧はプロパティとして持つことで、付けたり、外したりを容易にする。

### token

参照名を含む文字列をトークンに分ける。

各トークンは、`Identifier` をのぞき1文字。

- Comma: 参照名の区切り
- Whitespace: 本来なら読み飛ばし対象なので必要ないが、過去に Whitespace で区切っていたこともあり、読み飛ばす、区切りの両方の意味で用いるためトークン化
- CloseParen: 閉じ括弧
- OpenParen: 開き括弧
- Range: 範囲記号。`R1-2` の `-` に相当する
- Identifier: 上記に該当しない文字は全て連結して参照名識別子として扱う

### lexer

文字列を上記トークンに分解するためのモジュール。

### parser

各トークンの並びを解析し、個別の参照名(Designator)として分解する。

### builder

個別参照名配列をまとめ表現するためのモジュール。

## Examples

### まとめ文字列を分解

```rust
use designator::parser::Parser;

fn main() {
    let s = "R1~5,7,8,10,11";
    let mut parser = Parser::new(s);

    let designators = parser.parse();
    let mut iter = designators.into_iter();

    assert_eq!(iter.next(), Some("R1".to_string()));
    assert_eq!(iter.next(), Some("R2".to_string()));
    assert_eq!(iter.next(), Some("R3".to_string()));
	...
    assert_eq!(iter.next(), Some("R11".to_string()));
    assert_eq!(iter.next(), None);
}
```

### 個別参照名をまとめ

```rust
use designator::builder;

fn main() {
    let v = vec![
        "R1".to_string(),
        "R2".to_string(),
        "R3".to_string(),
        "R4".to_string(),
        "R5".to_string(),
        "R7".to_string(),
        "R8".to_string(),
        "R10".to_string(),
        "R11".to_string(),
    ];

    let s = builder::build(v);
    assert_eq!(s, "R1~5,7,8,10,11".to_string());
}
```