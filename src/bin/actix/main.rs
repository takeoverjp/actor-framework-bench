use actix::prelude::*;

mod heart_beat;
mod platform;
mod service;

use heart_beat::HeartBeatActor;
use platform::PlatformActor;
use service::ServiceActor;

#[actix::main]
async fn main() {
    println!("main started");
    // Start ServiceActor in new Arbiter
    let arbiter = Arbiter::new();
    arbiter.spawn_fn(|| {
        ServiceActor.start();
        HeartBeatActor.start();
        PlatformActor.start();
    });
    arbiter.join().unwrap();
}
