extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate num_cpus;
extern crate http;
extern crate cute_log;
#[macro_use]
extern crate log;

use std::net;
use std::cmp;

mod middleware;
mod proxy;

use actix_web::server::HttpServer;

fn application() -> actix_web::App<()> {
    actix_web::App::new().middleware(middleware::Logger)
                         .default_resource(|res| res.with_async(proxy::forward))
}

fn main() {
    let _ = cute_log::init();

    let addr = net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 8080);
    let cpu_num = cmp::max(num_cpus::get() / 2, 1);

    let system = actix::System::new("proxy");

    HttpServer::new(move || application()).bind(addr).expect("To bind HttpServer")
                                          .workers(cpu_num)
                                          .shutdown_timeout(5)
                                          .start();

    info!("Starts proxy at 0.0.0.0:8080");
    let _ = system.run();
}
