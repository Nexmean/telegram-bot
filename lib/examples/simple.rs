extern crate futures;
extern crate telegram_bot;
extern crate tokio;

use std::env;

use futures::{Future, Stream};
use telegram_bot::*;

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let api = Api::configure(token).build().unwrap();

    // Fetch new updates via long poll method
    let future = api.stream().for_each(move |update| {
        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                tokio::spawn(
                    api.send(message.text_reply(
                        format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                    ))
                        .map(|_| ())
                        .map_err(|e| panic!("Error during request: {:?}", e))
                );
            }
        }

        Ok(())
    })
        .map(|_| ())
        .map_err(|e| panic!("Error during taking updates: {:?}", e));

    tokio::run(future);
}
