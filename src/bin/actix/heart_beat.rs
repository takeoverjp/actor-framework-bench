use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
struct HeartBeat(String);

#[derive(Message)]
#[rtype(result = "()")]
struct HeartBeatNotifyActorStarted(String); // TODO: Recipient<HeartBeat>

#[derive(Message)]
#[rtype(result = "()")]
struct HeartBeatNotifyActorStopped(String); // TODO: Recipient<HeartBeat>

#[derive(Default)]
pub struct HeartBeatActor;

impl Actor for HeartBeatActor {
    type Context = Context<Self>;
}
impl actix::Supervised for HeartBeatActor {}

impl ArbiterService for HeartBeatActor {
    fn service_started(&mut self, ctx: &mut Context<Self>) {
        println!("HeartBeatActor started");

        ctx.run_interval(std::time::Duration::from_secs(1), |_, _| {
            println!("HeartBeat");
            // TODO: check all registered actors
        });
    }
}

impl Handler<HeartBeat> for HeartBeatActor {
    type Result = ();

    fn handle(&mut self, msg: HeartBeat, _: &mut Context<Self>) {
        println!("HeartBeat: {}", msg.0);
    }
}

impl Handler<HeartBeatNotifyActorStarted> for HeartBeatActor {
    type Result = ();

    fn handle(&mut self, msg: HeartBeatNotifyActorStarted, _: &mut Context<Self>) {
        println!("HeartBeatNotifyActorStarted: {}", msg.0);
    }
}

impl Handler<HeartBeatNotifyActorStopped> for HeartBeatActor {
    type Result = ();

    fn handle(&mut self, msg: HeartBeatNotifyActorStopped, _: &mut Context<Self>) {
        println!("HeartBeatNotifyActorStopped: {}", msg.0);
    }
}

pub fn notify_actor_started(name: &str) {
    // get HeartBeatActor address from the registry
    let hb = HeartBeatActor::from_registry();
    hb.do_send(HeartBeatNotifyActorStarted(String::from(name)));
}

pub fn notify_actor_stopped(name: &str) {
    // get HeartBeatActor address from the registry
    let hb = HeartBeatActor::from_registry();
    hb.do_send(HeartBeatNotifyActorStopped(String::from(name)));
}
