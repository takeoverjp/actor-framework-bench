use ractor::Actor;
use ractor::ActorProcessingErr;
use ractor::ActorRef;
use ractor::cast;

#[derive(Debug, Clone)]
pub struct HeartBeatActor;

#[derive(Debug, Clone)]
pub enum HeartBeatMessage {
    Tick,
}

impl Actor for HeartBeatActor {
    type Msg = HeartBeatMessage;

    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        let interval = tokio::time::interval(std::time::Duration::from_secs(1));
        tokio::spawn(async move {
            let mut interval = interval;
            loop {
                interval.tick().await;
                if cast!(myself, HeartBeatMessage::Tick).is_err() {
                    break;
                }
            }
        });
        Ok(())
    }

    async fn handle(
        &self,
        _: ActorRef<Self::Msg>,
        message: Self::Msg,
        _: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            HeartBeatMessage::Tick => {
                tracing::info!("HeartBeat");
            }
        }
        Ok(())
    }
}
