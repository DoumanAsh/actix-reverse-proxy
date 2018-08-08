use ::actix_web;
use ::actix_web::{HttpRequest, HttpResponse};
use ::actix_web::middleware::{Middleware, Started, Finished};

///Debug only logger.
///
///Prints to info category but disabled unless `debug_assertions` are on
pub struct Logger;

impl<S> Middleware<S> for Logger {
    #[allow(unused)]
    fn start(&self, req: &HttpRequest<S>) -> Result<Started, actix_web::Error> {
        #[cfg(debug_assertions)]
        {
            use ::actix_web::HttpMessage;
            use ::std::str;

            info!("HTTP: {remote} --> {method} {path} {version:?}\n{headers}\n",
                  remote=req.connection_info().remote().unwrap_or(""),
                  method=req.method(),
                  path=req.path(),
                  version=req.version(),
                  headers=req.headers().iter()
                                       .map(|(key, value)| format!("{}: {}\n", key.as_str(), str::from_utf8(value.as_bytes()).unwrap_or("Invalid UTF-8 value")))
                                       .collect::<String>()
                  );
        }
        Ok(Started::Done)
    }

    #[allow(unused)]
    fn finish(&self, _req: &HttpRequest<S>, resp: &HttpResponse) -> Finished {
        #[cfg(debug_assertions)]
        {
            use ::std::str;
            info!("HTTP: <-- {version:?} {status}{error}\n{headers}\n",
                  version=resp.version().unwrap_or(::http::Version::HTTP_11),
                  status=resp.status(),
                  error=resp.error().map(|error| format!(" Origin Error: {}", error)).unwrap_or("".to_string()),
                  headers=resp.headers().iter()
                                        .map(|(key, value)| format!("{}: {}\n", key.as_str(), str::from_utf8(value.as_bytes()).unwrap_or("Invalid UTF-8 value")))
                                        .collect::<String>()
                  );
        }

        Finished::Done
    }
}
