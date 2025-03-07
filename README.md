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

## 開発環境

- rustc
  - `1.84.1`
- cargo
  - `1.84.1`

## dependencies

- clap
  - `4.5.31`
  - features: `derive`
  - CLIのインターフェイスの実装に採用
- csv
  - `1.3.1`
  - CSVの読み書き
- chrono
  - `0.4.40`
  - features: `serde`
  - 日付処理
- serde
  - `1.0.198`
  - features: `derive`
  - データの入出力におけるシリアライズ／デシリアライズ
