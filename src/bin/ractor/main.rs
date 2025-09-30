mod heart_beat;
mod log;
mod ping_pong;
mod router;

use heart_beat::HeartBeatActor;
use ping_pong::PingPong;
use ractor::Actor;
use router::RouterActor;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    log::init();

    let (_router_actor, router_actor_handle) = Actor::spawn(None, RouterActor, ())
        .await
        .expect("Failed to start router actor");

    let (_actor, ping_pong_handle) = Actor::spawn(None, PingPong, ())
        .await
        .expect("Failed to start ping-pong actor");

    let (_actor, heart_beat_actor_handle) = Actor::spawn(None, HeartBeatActor, ())
        .await
        .expect("Failed to start heartbeat actor");

    ::tokio::try_join!(
        router_actor_handle,
        ping_pong_handle,
        heart_beat_actor_handle
    )
    .expect("Each actor should run without finish");
}
