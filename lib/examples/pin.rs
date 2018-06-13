extern crate futures;
extern crate telegram_bot;
extern crate tokio;

use std::env;

use futures::{Future, Stream};
use telegram_bot::*;

fn process(api: Api, message: Message) {
    if let MessageKind::Text { ref data, .. } = message.kind {
        match data.as_str() {
            "/pin" => {
                message.reply_to_message
                    .map(|message| tokio::spawn(api.send(message.pin()).map_err(|_| ())));
            }
            "/unpin" => {
                tokio::spawn(api.send(message.chat.unpin_message()).map_err(|_| ()));
            }
            _ => ()
        }
    }
}


fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();

    let api = Api::configure(token).build().unwrap();

    let future = api.stream().for_each(move |update| {
        if let UpdateKind::Message(message) = update.kind {
            process(api.clone(), message)
        }
        Ok(())
    })
        .map_err(|e| panic!("Error during taking updates: {:?}", e));

    tokio::run(future);
}
