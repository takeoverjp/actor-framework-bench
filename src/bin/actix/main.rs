mod heart_beat;
mod platform;
mod service;

use crate::heart_beat::HeartBeatActor;
use crate::platform::PlatformActor;
use crate::service::ServiceActor;
use ::actix::prelude::*;

#[actix::main]
async fn main() {
    println!("main started");
    // Start ServiceActor in current Arbiter
    actix::spawn(async {
        ServiceActor.start();
        HeartBeatActor.start();
        PlatformActor.start();

        // run actors until Ctrl-C
        ::tokio::signal::ctrl_c().await.unwrap();
    })
    .await
    .unwrap();
}
