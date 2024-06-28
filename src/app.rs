use anyhow::Result;
use hello_world::greeter_server::{Greeter, GreeterServer};
use std::net::{Ipv4Addr, SocketAddr};
use tonic::transport::{server::Router, Server};
use tonic_health::server::HealthReporter;

use crate::config::Config;

pub struct App {
    router: Router,
    addr: SocketAddr,
    health_reporter: HealthReporter,
}

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: tonic::Request<hello_world::HelloRequest>,
    ) -> Result<tonic::Response<hello_world::HelloReply>, tonic::Status> {
        tracing::info!("Got a request: {:?}", request);
        let reply = hello_world::HelloReply {
            message: format!("Hello, {}!", request.into_inner().name),
        };
        Ok(tonic::Response::new(reply))
    }
}

impl App {
    pub async fn new(config: &Config) -> Self {
        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();

        health_reporter
            .set_serving::<GreeterServer<MyGreeter>>()
            .await;

        let router = Server::builder()
            .add_service(GreeterServer::new(MyGreeter::default()))
            .add_service(health_service);
        let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, config.port));

        Self {
            router,
            addr,
            health_reporter,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        let health_reporter = &mut self.health_reporter;

        self.router
            .serve_with_shutdown(self.addr, async {
                #[cfg(unix)]
                use tokio::signal::unix::{signal, SignalKind};
                let mut sigterm =
                    signal(SignalKind::terminate()).expect("failed to register signal handler");
                let mut sigint =
                    signal(SignalKind::interrupt()).expect("failed to register signal handler");
                let mut sigquit =
                    signal(SignalKind::quit()).expect("failed to register signal handler");

                tokio::select! {
                  _ = sigterm.recv() => tracing::info!("received SIGTERM"),
                  _ = sigint.recv() => tracing::info!("received SIGINT"),
                  _ = sigquit.recv() => tracing::info!("received SIGQUIT"),
                }

                health_reporter
                    .set_not_serving::<GreeterServer<MyGreeter>>()
                    .await;

                tracing::info!("all services terminated");
            })
            .await?;
        Ok(())
    }
}
