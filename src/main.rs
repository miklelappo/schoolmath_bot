use crate::exgen::{AddFormat, DivFormat, MulFormat, SubFormat, generate_excercises};
use log::info;
use teloxide::{prelude::*, utils::command::BotCommands};

mod exgen;

#[derive(BotCommands, Clone)]
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
            Command::Addition => {
                let response = generate_excercises(1000_u16, 10_usize)
                    .into_iter()
                    .fold(String::new(), |acc, n| acc.to_owned() + &n.fmt_add() + "\n");
                bot.send_message(msg.chat.id, response).await?
            }
            Command::Subtraction => {
                let response = generate_excercises(1000_u16, 10_usize)
                    .into_iter()
                    .fold(String::new(), |acc, n| acc.to_owned() + &n.fmt_sub() + "\n");
                bot.send_message(msg.chat.id, response).await?
            }
            Command::Multiplication => {
                let response = generate_excercises(10_u16, 20_usize)
                    .into_iter()
                    .fold(String::new(), |acc, n| acc.to_owned() + &n.fmt_mul() + "\n");
                bot.send_message(msg.chat.id, response).await?
            }
            Command::Division => {
                let response = generate_excercises(10_u16, 20_usize)
                    .into_iter()
                    .fold(String::new(), |acc, n| acc.to_owned() + &n.fmt_div() + "\n");
                bot.send_message(msg.chat.id, response).await?
            }
        };
        Ok(())
    })
    .await;
}
