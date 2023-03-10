use std::{io, net::TcpListener};

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run, telemetry};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection)?.await
}
