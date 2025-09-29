mod heart_beat;
mod log;
mod ping_pong;

use heart_beat::HeartBeatActor;
use ping_pong::PingPong;
use ractor::Actor;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    log::init();

    let (_actor, handle) = Actor::spawn(None, PingPong, ())
        .await
        .expect("Failed to start ping-pong actor");
    handle
        .await
        .expect("Ping-pong actor failed to exit properly");

    let (_actor, handle) = Actor::spawn(None, HeartBeatActor, ())
        .await
        .expect("Failed to start heartbeat actor");
    handle
        .await
        .expect("Heartbeat actor failed to exit properly");
}
