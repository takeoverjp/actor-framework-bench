use ractor::Actor;
use ractor::ActorProcessingErr;
use ractor::ActorRef;
use ractor::cast;

pub struct PingPong;

#[derive(Debug, Clone)]
pub enum Message {
    Ping,
    Pong,
}
impl Message {
    fn next(&self) -> Self {
        match self {
            Self::Ping => Self::Pong,
            Self::Pong => Self::Ping,
        }
    }

    fn print(&self) {
        match self {
            Self::Ping => tracing::info!("ping.."),
            Self::Pong => tracing::info!("pong.."),
        }
    }
}

impl Actor for PingPong {
    type Msg = Message;

    type State = u8;
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        // startup the event processing
        cast!(myself, Message::Ping).unwrap();
        // create the initial state
        Ok(0u8)
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if *state < 10u8 {
            message.print();
            cast!(myself, message.next()).unwrap();
            *state += 1;
        } else {
            tracing::info!("");
            myself.stop(None);
        }
        Ok(())
    }
}
