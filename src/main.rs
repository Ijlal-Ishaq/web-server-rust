mod routers {
    pub mod base;
    pub mod books;
}
mod services {
    pub mod books;
}
mod utils {
    pub mod helpers;
}
mod mongo;

use std::env;
use std::net::SocketAddr;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use routers::base::base_router;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

    let port = env::var("PORT")
        .expect("set the PORT environment var")
        .parse::<u16>()
        .expect("error parsing PORT");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(base_router))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
