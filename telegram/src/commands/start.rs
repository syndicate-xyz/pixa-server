use entity::tg_user::ActiveModel;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{GlobalCommand, HandlerResult};

pub async fn start(bot: Bot, message: Message) -> HandlerResult {
    let db = entity::get_db();
    let user = ActiveModel {
        id: Set(message.from().unwrap().id.clone_from(source);),
        username: Set(message.from().unwrap().username.clone()),
        first_name: Set(message.from().unwrap().first_name.clone()),
        ..Default::default()
    };

    bot.send_message(message.chat.id, GlobalCommand::descriptions().to_string())
        .await?;
    Ok(())
}
