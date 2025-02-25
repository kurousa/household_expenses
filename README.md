# 家計簿CLIアプリ

## 仕様

- 「収支」を管理する単位として「口座」を作成できる
  - 収支は、日付、用途、金額を記録可
  - 作成した口座はCSVとして出力される
    - kakeibo new <accout_name> で空のCSVを出力
- 「口座」に「入金」を記録できる
- 「口座」に「出金」を記録できる
- CSVから口座に一括で登録ができる
- レポートを出力できる

## 利用できるコマンド

- kakeibo new <account_name>
  - 新たな口座を作成
- kakeibo deposit
  - 入金についての操作
- kakeibo withdraw
  - 出金についての操作
- kakeibo import
  - CSVからのインポート操作
- kakeibo report
  - レポートの出力

## dependencies

- clap
  - `4.5.31`
  - CLIのインターフェイスの実装に採用