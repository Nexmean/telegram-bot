extern crate futures;
extern crate tokio;
extern crate telegram_bot;

use futures::{Future, Stream};

use telegram_bot::*;

#[test]
fn test_simple() {
    let token = "526131894:AAErjGPa6AL2fArwqPXT2DrhtUsQnO9emNQ";

    let api = telegram_bot::Api::configure(token).build().unwrap();

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
                        .then(|_| Ok(()))
                );
            }
        }

        Ok(())
    })
        .then(|x| {
            println!("{:?}", x);
            Ok(())
        });


    tokio::run(future);
}