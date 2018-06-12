//! Connector with hyper backend.

use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use futures::{Future, Stream};
use futures::future::{self, result};
use hyper;
use hyper::{Method, Uri};
use hyper::client::Client;
use hyper::client::connect::Connect;
use hyper_tls::HttpsConnector;

use telegram_bot_raw::{HttpRequest, HttpResponse, Method as TelegramMethod, Body as TelegramBody};

use errors::Error;
use future::{TelegramFuture, NewTelegramFuture};

use super::_base::Connector;

/// This connector uses `hyper` backend.
pub struct HyperConnector<C> {
    inner: Arc<Client<C>>
}

impl<C> fmt::Debug for HyperConnector<C> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        "hyper connector".fmt(formatter)
    }
}

impl<C> HyperConnector<C> {
    pub fn new(client: Client<C>) -> Self {
        HyperConnector {
            inner: Arc::new(client)
        }
    }
}

impl<C: Connect + 'static> Connector for HyperConnector<C> {
    fn request(&self, token: &str, req: HttpRequest) -> TelegramFuture<HttpResponse> {
        let uri = match Uri::from_str(&req.url.url(token)) {
            Ok(uri) => uri,
            Err(error) => return TelegramFuture::new(Box::new(future::err(error.into())))
        };

        let client = self.inner.clone();

        let method = match req.method {
            TelegramMethod::Get => Method::GET,
            TelegramMethod::Post => Method::POST,
        };

        let mut request_builder = hyper::Request::builder();
        request_builder
            .method(method)
            .uri(uri);

        let http_request = match req.body {
            TelegramBody::Empty => request_builder.body(hyper::Body::empty()),
            TelegramBody::Json(body) => request_builder
                .header("Content-Type", "application/json")
                .body(body.into()),
            body => panic!("Unknown body type {:?}", body),
        };

        let request = result(http_request).map_err(|e| -> Error { e.into() })
            .and_then(move |http_request| {
                client.request(http_request).map_err(From::from)
            });

        let future = request
            .and_then(move |response| {
                response.into_body().map_err(From::from)
                    .fold(vec![], |mut result, chunk| -> Result<Vec<u8>, Error> {
                        result.extend_from_slice(&chunk);
                        Ok(result)
                    })
            })
            .and_then(|body| {
                Ok(HttpResponse {
                    body: Some(body),
                })
            });

        TelegramFuture::new(Box::new(future))
    }
}

/// Returns default hyper connector. Uses one resolve thread and `HttpsConnector`.
pub fn default_connector() -> Result<Box<Connector>, Error> {
    let connector = HttpsConnector::new(1)?;
    let client = Client::builder()
        .build::<_, hyper::Body>(connector);
    Ok(Box::new(HyperConnector::new(client)))
}
