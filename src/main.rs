use crate::exgen::{OpSign, generate_excercises};
use anyhow::Result;
use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};
use tempfile::Builder;

mod exgen;
mod pdf;

#[derive(BotCommands, Clone, Hash, Eq, PartialEq)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
enum Command {
    #[command(description = "Show available commands.")]
    Help,
    #[command(description = "Start the bot.")]
    Start,
    #[command(description = "Get 10 addition excercises")]
    Addition,
    #[command(description = "Get 10 subtraction excercises")]
    Subtraction,
    #[command(description = "Get 20 multiplication excercises")]
    Multiplication,
    #[command(description = "Get 10 division excercises")]
    Division,
    #[command(description = "Get daily PDF for 2nd class")]
    Daily,
    #[command(description = "Get written addition/subtraction PDF")]
    WrittenDaily,
}

lazy_static! {
    static ref ExerciseParams: HashMap<Command, (OpSign, u32, usize)> = {
        let mut m = HashMap::new();
        m.insert(Command::Addition, (OpSign::Add, 1000_u32, 10_usize));
        m.insert(Command::Subtraction, (OpSign::Sub, 1000_u32, 10_usize));
        m.insert(Command::Multiplication, (OpSign::Mul, 10_u32, 20_usize));
        m.insert(Command::Division, (OpSign::Div, 10_u32, 20_usize));
        m
    };
}

fn response(cmd: Command) -> String {
    if let Some((sign, max_value, count)) = ExerciseParams.get(&cmd) {
        generate_excercises(*sign, 1..=*max_value, *count)
            .into_iter()
            .fold(String::new(), |acc, n| {
                acc.to_owned() + &format!("{n}") + "\n"
            })
    } else {
        "unknown command".to_string()
    }
}

fn render_to_file(style: &str, path: &str) -> Result<()> {
    let mut file = tempfile::Builder::new().suffix(".pdf").tempfile()?;
    match style {
        "daily" => pdf::pdf(&mut file),
        "written" => pdf::pdf_written(&mut file),
        other => anyhow::bail!("unknown style {other:?}: use 'daily' or 'written'"),
    }
    let bytes = std::fs::read(file.path())?;
    File::create(path)?.write_all(&bytes)?;
    println!("wrote {path}");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "render" {
        let style = args.get(2).map(String::as_str).unwrap_or("daily");
        let default_path = format!("{style}.pdf");
        let path = args.get(3).map(String::as_str).unwrap_or(&default_path);
        return render_to_file(style, path);
    }

    pretty_env_logger::init();
    info!("Starting Telegram bot...");

    let bot = Bot::from_env();

    Command::repl(bot, |bot: Bot, msg: Message, cmd| async move {
        match cmd {
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?
            }
            Command::Start => {
                bot.send_message(
                    msg.chat.id,
                    "Welcome to SchoolMath! Type /help to see commands.",
                )
                .await?
            }
            Command::Daily => {
                let mut temp_pdf = Builder::new()
                    .prefix("daily_pdf")
                    .suffix(".pdf")
                    .tempfile()?;
                let path = temp_pdf.path().to_path_buf();
                pdf::pdf(&mut temp_pdf);
                bot.send_document(msg.chat.id, InputFile::file(path))
                    .await?
            }
            Command::WrittenDaily => {
                let mut temp_pdf = Builder::new()
                    .prefix("written_pdf")
                    .suffix(".pdf")
                    .tempfile()?;
                let path = temp_pdf.path().to_path_buf();
                pdf::pdf_written(&mut temp_pdf);
                bot.send_document(msg.chat.id, InputFile::file(path))
                    .await?
            }
            cmd => bot.send_message(msg.chat.id, response(cmd)).await?,
        };
        Ok(())
    })
    .await;
    Ok(())
}
