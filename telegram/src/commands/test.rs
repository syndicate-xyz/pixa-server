use std::sync::Arc;
use teloxide::{
    dispatching::{
        dialogue::{GetChatId, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup, ParseMode,
        ReplyMarkup,
    },
    utils::markdown::bold,
};

use crate::{GlobalCommand, GlobalDialogue, GlobalState, HandlerResult};

#[derive(Clone, Default, Debug)]

pub enum State {
    #[default]
    Test,
}

async fn settings(bot: Bot, message: Message) -> HandlerResult {
    bot.send_message(message.chat.id, "_Please select from an option below_")
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(create_main_markup())
        .await?;
    Ok(())
}

async fn callback_handler(
    bot: Bot,
    callback: CallbackQuery,
    dialogue: GlobalDialogue,
    // client: DataClient,
) -> anyhow::Result<()> {
    match callback.data {
        Some(data) => match data.as_str() {
            "test" => {
                if let Some(message) = callback.message {
                    bot.delete_message(message.chat().id, message.id()).await?;
                    println!("Test callback triggered, changing state to Test(State::Test)");
                    let message = bot
                        .send_message(message.chat().id, "Changing name")
                        .parse_mode(ParseMode::MarkdownV2)
                        .await?;

                    dialogue.update(GlobalState::Test(State::Test)).await?;
                    println!("State updated to Test(State::Test)");
                }
            }
            _ => {}
        },

        None => {}
    };

    Ok(())
}

async fn change_name(
    bot: Bot,
    message: Message,
    dialogue: GlobalDialogue,
    // client: Arc<DataClient>,
) -> anyhow::Result<()> {
    println!(
        "change_name function called with message: {:?}",
        message.text()
    );
    let send_message = |text: &str| {
        bot.send_message(message.chat.id, text)
            .parse_mode(ParseMode::MarkdownV2)
    };

    match message.text() {
        Some(text) => {
            println!("Text received: '{}'", text);
            if matches!(text, "Cancel" | "cancel") {
                send_message("_Change user name request_").await?;
                dialogue.update(GlobalState::Idle).await?;
            } else {
                let message = send_message("_Please wait for a moment_")
                    .reply_markup(ReplyMarkup::kb_remove())
                    .await?;

                println!("Text {}", text);

                if text.trim() != "" {
                    let msg = &format!("Name successfully changed to {}", bold(text));
                    println!("Text is not empty, updating name: {}", msg);
                    // bot.edit_message_text(message.chat.id, message.id, msg)
                    // .parse_mode(ParseMode::MarkdownV2)
                    // .await?;

                    bot.delete_message(message.chat.id, message.id).await?;

                    send_message(&format!("Name successfully changed to {}", text)).await?;

                    println!("Updating state to Idle");
                    dialogue.update(GlobalState::Idle).await?;
                } else {
                    println!("Text is empty");
                    send_message("Please enter a valid name").await?;
                }
            }
        }
        None => {
            println!("No text in message");
            send_message("_Invalid reply please try again using text only_").await?;
        }
    };

    Ok(())
}

pub fn schema() -> UpdateHandler<anyhow::Error> {
    dptree::entry()
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, InMemStorage<GlobalState>, GlobalState>()
                .branch(dptree::case![GlobalState::Idle].endpoint(callback_handler)),
        )
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<GlobalState>, GlobalState>()
                .branch(
                    dptree::case![GlobalState::Idle]
                        .filter_command::<GlobalCommand>()
                        .branch(dptree::case![GlobalCommand::Test].endpoint(settings)),
                )
                .branch(
                    dptree::case![GlobalState::Test(x)]
                        .branch(dptree::case![State::Test].endpoint(change_name)),
                ),
        )
}

fn create_main_markup() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new([[InlineKeyboardButton::callback("Test", "test")]])
}
