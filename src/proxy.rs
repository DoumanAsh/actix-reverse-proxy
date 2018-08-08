use ::actix_web;
use ::http;

use actix_web::{HttpRequest, HttpResponse, HttpMessage, client};
use ::futures::{Stream, Future};

const FORWARD_URL: &'static str = "https://github.com";

pub fn forward(req: HttpRequest) -> impl Future<Item=actix_web::HttpResponse, Error=actix_web::Error>  {
    let forward_uri = match req.uri().query() {
        Some(query) => format!("{}{}?{}", FORWARD_URL, req.uri().path(), query),
        None => format!("{}{}", FORWARD_URL, req.uri().path()),
    };

    let mut forward_req = client::ClientRequest::build_from(&req);
    forward_req.uri(forward_uri.as_str());
    forward_req.set_header(http::header::HOST, &FORWARD_URL[8..]);

    forward_req.finish().expect("To create valid forward request")
               .send()
               .map_err(|error| {
                   error!("Error: {}", error);
                   error.into()
               })
               .map(|resp| {
                   let mut back_rsp = HttpResponse::build(resp.status());
                   for (key, value) in resp.headers() {
                       back_rsp.header(key.clone(), value.clone());
                   }

                   let back_body = resp.payload().from_err();
                   back_rsp.body(actix_web::Body::Streaming(Box::new(back_body)))
               })
}
