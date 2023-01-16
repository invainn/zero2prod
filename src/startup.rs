use std::{io, net::TcpListener};

use actix_web::{dev::Server, App, HttpServer};

use crate::routes;

pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(routes::subscribe)
            .service(routes::health_check)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
