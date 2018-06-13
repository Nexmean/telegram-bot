extern crate futures;
extern crate telegram_bot;
extern crate tokio;

use std::env;
use futures::Future;

use telegram_bot::{Api, GetMe};

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();

    let api = Api::configure(token).build().unwrap();
    let future = api.send(GetMe);

    tokio::run(
        future
            .map(|user| println!("{:?}", user))
            .map_err(|e| panic!("Error during request: {:?}", e))
    );
}
