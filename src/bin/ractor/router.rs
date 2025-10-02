use crate::heart_beat::HeartBeatActor;
use crate::heart_beat::HeartBeatArguments;
use crate::heart_beat::HeartBeatMessage;
use ::ractor::Actor;
use ::ractor::ActorProcessingErr;
use ::ractor::ActorRef;

#[derive(Debug, Clone)]
pub struct RouterActor;

#[derive(Debug, Clone)]
pub enum RouterMessage {
    HeartBeat(HeartBeatMessage),
}

pub struct RouterState {
    heart_beat_actor: ActorRef<HeartBeatMessage>,
}

impl Actor for RouterActor {
    type Msg = RouterMessage;

    type State = RouterState;
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let (heart_beat_actor, _heart_beat_actor_handle) =
            Actor::spawn(None, HeartBeatActor, HeartBeatArguments::new(myself))
                .await
                .expect("Failed to start heartbeat actor");
        Ok(RouterState { heart_beat_actor })
    }

    async fn handle(
        &self,
        _: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        // Routing messages
        match message {
            RouterMessage::HeartBeat(msg) => {
                ::tracing::info!("Router received HeartBeat message: {:?}", msg);
                ractor::cast!(state.heart_beat_actor, msg).unwrap();
            }
        }
        Ok(())
    }
}
