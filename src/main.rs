use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use csv::{Reader, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::OpenOptions};

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

/// CSVインポート用の構造体
/// 不正な入力受付を防ぐ
#[derive(Serialize, Deserialize)]
struct Record {
    日付: NaiveDate,
    用途: String,
    金額: i32,
}
#[derive(Args)]
struct ImportArgs {
    /// インポート対象ファイル名
    src_file_name: String,
    /// インポート先口座名
    dst_account_name: String,
}
impl ImportArgs {
    fn run(&self) {
        dbg!(&self.src_file_name);
        dbg!(&self.dst_account_name);
        // 追記モードでファイルを開く設定
        let open_options = OpenOptions::new()
            .append(true)
            .open(format!("{}.csv", self.dst_account_name))
            .unwrap();
        // 作成したOpenOptionsを利用してwriterを作成
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_writer(open_options);
        // CSVファイルを読み込み、行ごとにインポート先CSVに書き込む
        let mut reader = Reader::from_path(&self.src_file_name).unwrap();
        for result in reader.deserialize() {
            let record: Record = result.unwrap();
            writer.serialize(record).unwrap();
        }
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct ReportArgs {
    /// インポート対象ファイル名
    files: Vec<String>,
}
impl ReportArgs {
    fn run(&self) {
        let mut map = HashMap::new();
        for file in &self.files {
            let mut reader = Reader::from_path(file).unwrap();
            for result in reader.deserialize() {
                let record: Record = result.unwrap();
                let amount: i32 = record.金額;
                let date: NaiveDate = record.日付;
                let sum = map.entry(date.format("%Y-%m").to_string()).or_insert(0);
                *sum += amount;
            }
        }
        println!("{:?}", map)
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
    Import(ImportArgs),
    /// レポート出力
    Report(ReportArgs),
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
        Command::Import(args) => args.run(),
        Command::Report(args) => args.run(),
    }
}
