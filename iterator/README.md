# Iterator

## Iterator とは？

反復可能なデータ構造の抽象概念。繰り返す概念としてIteratorを導入することで、そのデータ構造の繰り返しのロジックを分離している。

## Examples

| example                                          | note                             |
| ------------------------------------------------ | -------------------------------- |
| [iterator_fibnacci](./example/iterator_fibnacci) | フィボナッチ数列                 |
| [iterator_garapon](./example/iterator_garapon)   | ガラポンもイテレーターですかね？ |

## どんな Iterator がある？

繰り返しを表現できるものなら何でも。C++20 以降は下記がある。

- ランダムアクセスイテレータ
- 双方向イテレータ
- 前方向イテレータ
- 出力イテレータ
- 入力イテレータ
- 隣接イテレータ

## どんなものが Iterator を持ってる？

- コレクション
  - vector
  - list
  - tree
  - hashmap
- 数列・集合
  - なにかの集合
  - なにかの数列(無限に続くものも)
  - フィボナッチ数列
- ストリーム
  - I/Oストリーム
- ランダムで要素が取り出せるもの
  - ガチャ
  - 福引のガラポン

もありそう。

## Refs
- [マンガでわかる Iterator](https://qiita.com/tanakahisateru/items/0a2c3cd2c3af1459902f)
- [cpprefjp - C++日本語リファレンス iterator](https://cpprefjp.github.io/reference/iterator.html)
