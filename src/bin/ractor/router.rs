use crate::heart_beat::HeartBeatActor;
use ractor::Actor;
use ractor::ActorProcessingErr;
use ractor::ActorRef;

#[derive(Debug, Clone)]
pub struct RouterActor;

#[derive(Debug, Clone)]
pub enum RouterMessage {}

impl Actor for RouterActor {
    type Msg = RouterMessage;

    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        let (_actor, _heart_beat_actor_handle) = Actor::spawn(None, HeartBeatActor, ())
            .await
            .expect("Failed to start heartbeat actor");
        Ok(())
    }

    async fn handle(
        &self,
        _: ActorRef<Self::Msg>,
        _message: Self::Msg,
        _: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        // Routing messages
        // match message {
        //     RouterMessage::Tick => {
        //         tracing::info!("Router");
        //     }
        // }
        Ok(())
    }
}
