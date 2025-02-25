use clap::{Args, Parser, Subcommand};
use csv::Writer;

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
#[derive(Subcommand)]
enum Command {
    /// 新しい口座を作成
    New(NewArgs),
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
    let args: App = App::parse();
    match args.command {
        Command::New(args) => args.run(),
        Command::Deposit => unimplemented!("Under construction"),
        Command::Withdraw => unimplemented!("Under construction"),
        Command::Import => unimplemented!("Under construction"),
        Command::Report => unimplemented!("Under construction"),
    }
}
