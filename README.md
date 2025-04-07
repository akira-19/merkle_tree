# merkle_tree

# merkle root test

```
cargo test -- --nocapture test_merkle_tree_with_odd_number_of_leaves
```

# API

1. merkle root

```
/users/root
```

2. merkle proof

```
/users/{id}/merkle_proof
```

# libraries

- sha2
  - hash ライブラリ
  - よくメンテナンスされており、sha2 に関しては現状攻撃報告はない
- hex
  - hex のエンコード/デコードをシンプルに行うライブラリ
  - メンテもされており、コード自体もシンプルなため採用
- actix-web
  - web framework で最も人気のものの 1 つ
- anyhow
  - thiserror とともにエラーハンドリングのデファクトスタンダード
  - 今回の仕様では厳密なエラーハンドリングは扱わないため anyhow
- rusqlite
  - sqlite をシンプルに扱うため
- serde, serde_json
  - rust のシリアライズ/デシリアライズのデファクトスタンダード

# Others

- main 関数は今回は unwrap でハンドリングしているが、エラー処理を行う
- エラーログの出力
- 独自のエラー型の作成
- テストの作成
  - 最低限 integration test は作成
- コード規模が大きくなる予定であればモジュールの細分化を行う
  - 現状 in-memory の db なので、datastore の抽象化して、db を変えられるようにしておく
  - User Domain を作成する
- データベースのコネクションプールの追加
- merkle tree の永続化をファイルではなく、DB や KVS に保存する
  - もっと大きくなった場合、json ではなくバイナリにした方が高速化可能
- User が増えた場合は Merkle Tree の再計算を並列化する
- Leaf の追加のメソッドの追加
- CI/CD の追加
- 認証認可の追加
