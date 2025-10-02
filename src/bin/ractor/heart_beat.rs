use crate::router::RouterMessage;
use ::ractor::Actor;
use ::ractor::ActorProcessingErr;
use ::ractor::ActorRef;
use ::ractor::cast;

#[derive(Debug, Clone)]
pub struct HeartBeatActor;

#[derive(Debug, Clone)]
pub enum HeartBeatMessage {
    Tick,
}

pub struct HeartBeatArguments {
    router: ActorRef<RouterMessage>,
}

impl HeartBeatArguments {
    pub fn new(router: ActorRef<RouterMessage>) -> Self {
        Self { router }
    }
}

impl Actor for HeartBeatActor {
    type Msg = HeartBeatMessage;

    type State = ();
    type Arguments = HeartBeatArguments;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let interval = tokio::time::interval(std::time::Duration::from_secs(1));
        tokio::spawn(async move {
            let mut interval = interval;
            loop {
                interval.tick().await;
                if cast!(
                    args.router,
                    RouterMessage::HeartBeat(HeartBeatMessage::Tick)
                )
                .is_err()
                {
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
                ::tracing::info!("HeartBeat");
            }
        }
        Ok(())
    }
}
