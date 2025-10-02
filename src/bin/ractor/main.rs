mod heart_beat;
mod log;
mod router;

use ractor::Actor;
use router::RouterActor;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    log::init();

    let (_router_actor, router_actor_handle) = Actor::spawn(None, RouterActor, ())
        .await
        .expect("Failed to start router actor");

    ::tokio::try_join!(router_actor_handle).expect("Each actor should run without finish");
}
