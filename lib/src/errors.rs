use telegram_bot_raw;

error_chain! {
    foreign_links {
        Url(::http::uri::InvalidUri);
        Http(::http::Error);
        Hyper(::hyper::Error);
        HyperTls(::hyper_tls::Error);
        Io(::std::io::Error);
        Timer(::tokio::timer::Error);
    }

    links {
        Raw(telegram_bot_raw::Error, telegram_bot_raw::ErrorKind);
    }
}
