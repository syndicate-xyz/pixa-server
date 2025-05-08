mod commands;
use commands::{message::handle_message, start};
use entity::{tg_user, tg_user::Entity as TgUser};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    payloads::SetWebhook,
    prelude::*,
    utils::command::{self, BotCommands},
};
use tracing::instrument::WithSubscriber;
use url::Url;

#[derive(Clone, Default, Debug)]
pub enum GlobalState {
    #[default]
    Idle,
    Start,
    Test(commands::test::State),
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum GlobalCommand {
    #[command(description = "start bot.")]
    Start,
    #[command(description = "testing")]
    Test,
    #[command(description = "display this text.")]
    Help,
}

pub type GlobalDialogue = Dialogue<GlobalState, InMemStorage<GlobalState>>;
type HandlerResult = Result<(), anyhow::Error>;
// type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn telegram_bot_entrypoint() {
    // let tg_user = tg_user

    let bot = Bot::from_env();
    let bot_clone = bot.clone();

    // const WEBHOOK_URL: &str = "https://api.vybenetwork.xyz/telegram/webhook";
    // let wh = SetWebhook::new(Url::parse(WEBHOOK_URL).unwrap());

    // GlobalCommand::repl(bot, answer).await;
    Dispatcher::builder(
        bot,
        dptree::entry()
            .branch(
                Update::filter_message()
                    .filter_command::<GlobalCommand>()
                    .branch(dptree::case![GlobalCommand::Start].endpoint(commands::start::start)),
            )
            .branch(commands::test::schema()),
    )
    .dependencies(dptree::deps![InMemStorage::<GlobalState>::new()])
    .default_handler(move |upd| {
        tracing::info!("Unhandled update");
        let bot = bot_clone.clone();
        async move {
            // tracing::warn!("Unhandled update: {:?}", upd);
            if let teloxide::types::UpdateKind::Message(message) = &upd.kind {
                let _ = handle_message(bot, message.clone()).await;
            }
        }
    })
    // If the dispatcher fails for some reason, execute this handler
    .error_handler(LoggingErrorHandler::with_custom_text(
        "An error has occurred in the dispatcher",
    ))
    // .distribution_function(|_| None::<()>)
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

// async fn answer(bot: Bot, msg: Message, cmd: GlobalCommand) -> ResponseResult<()> {
//     match cmd {
//         GlobalCommand::Start => {
//             bot.send_message(msg.chat.id, GlobalCommand::descriptions().to_string())
//                 .await?
//         }
//         GlobalCommand::Help => {
//             bot.send_message(msg.chat.id, GlobalCommand::descriptions().to_string())
//                 .await?
//         }
//     };

//     Ok(())
// }
