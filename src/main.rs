use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use csv::Writer;
use std::fs::OpenOptions;

#[derive(Args)]
struct NewArgs {
    /// 口座名
    account_name: String,
}
impl NewArgs {
    fn run(&self) {
        let file_name: String = format!("{}.csv", self.account_name);
        let mut writer: Writer<std::fs::File> = Writer::from_path(file_name).unwrap();
        writer.write_record(["日付", "用途", "金額"]).unwrap();
        writer.flush().unwrap();
        println!("新しい口座「{}」を作成しました", self.account_name)
    }
}
#[derive(Args)]
struct DepositArgs {
    /// 口座名
    account_name: String,
    /// 日付
    date: NaiveDate,
    /// 用途
    usage: String,
    /// 金額
    amount: u32,
}
impl DepositArgs {
    fn run(&self) {
        // 追記モードでファイルを開く設定
        let open_options = OpenOptions::new()
            .append(true)
            .open(format!("{}.csv", self.account_name))
            .unwrap();
        // 作成したOpenOptionsを利用してwriterを作成
        let mut writer = Writer::from_writer(open_options);

        writer
            .write_record(&[
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                self.amount.to_string(),
            ])
            .unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct WithDrawArgs {
    /// 口座名
    account_name: String,
    /// 日付
    date: NaiveDate,
    /// 用途
    usage: String,
    /// 金額
    amount: u32,
}
impl WithDrawArgs {
    fn run(&self) {
        // 追記モードでファイルを開く設定
        let open_options = OpenOptions::new()
            .append(true)
            .open(format!("{}.csv", self.account_name))
            .unwrap();
        // 作成したOpenOptionsを利用してwriterを作成
        let mut writer = Writer::from_writer(open_options);

        writer
            .write_record(&[
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                format!("-{}", self.amount),
            ])
            .unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Subcommand)]
enum Command {
    /// 新しい口座を作成
    New(NewArgs),
    /// 口座に入金する
    Deposit(DepositArgs),
    /// 口座から出金する
    Withdraw(WithDrawArgs),
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
    let args: App = App::parse();
    match args.command {
        Command::New(args) => args.run(),
        Command::Deposit(args) => args.run(),
        Command::Withdraw(args) => args.run(),
        Command::Import => unimplemented!("Under construction"),
        Command::Report => unimplemented!("Under construction"),
    }
}
