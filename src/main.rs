use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    /// 新しい口座を作成
    New,
    /// 口座に入金する
    Deposit,
    /// 口座から出金する
    Withdraw,
    /// CSVからインポートする
    Import,
    /// レポート出力
    Report,
}

#[derive(Parser)]
#[clap(name = "kakeibo", version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}
fn main() {
    // clapによるパース
    let _args = App::parse();
}
