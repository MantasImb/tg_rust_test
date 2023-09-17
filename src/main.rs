use teloxide::{
    prelude::{AutoSend, Bot},
    requests::{Requester, RequesterExt},
    respond,
    types::{ChatId, Message},
};

#[tokio::main]
// Returning result from main allows for easier error handling
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = Bot::from_env().auto_send();
    let user = ChatId(1494843341);
    bot.send_message(user, "Hi!").await?;

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        if let Some(text) = message.text() {
            bot.send_message(message.chat.id, text).await?;
        }
        respond(())
    })
    .await;
    Ok(())
}
