use crate::exgen::{OpSign, generate_excercises};
use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use teloxide::{prelude::*, utils::command::BotCommands};

mod exgen;

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
}

lazy_static! {
    static ref ExerciseParams: HashMap<Command, (OpSign, u16, usize)> = {
        let mut m = HashMap::new();
        m.insert(Command::Addition, (OpSign::Add, 1000_u16, 10_usize));
        m.insert(Command::Subtraction, (OpSign::Sub, 1000_u16, 10_usize));
        m.insert(Command::Multiplication, (OpSign::Mul, 10_u16, 20_usize));
        m.insert(Command::Division, (OpSign::Div, 10_u16, 20_usize));
        m
    };
}

fn response(cmd: Command) -> String {
    if let Some((sign, max_value, count)) = ExerciseParams.get(&cmd) {
        generate_excercises(*sign, *max_value, *count)
            .into_iter()
            .fold(String::new(), |acc, n| {
                acc.to_owned() + &format!("{n}") + "\n"
            })
    } else {
        "unknown command".to_string()
    }
}

#[tokio::main]
async fn main() {
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
            cmd => bot.send_message(msg.chat.id, response(cmd)).await?,
        };
        Ok(())
    })
    .await;
}
