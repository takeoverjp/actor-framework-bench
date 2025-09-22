use actix::prelude::*;

pub struct ServiceActor;

use crate::heart_beat;

const NAME: &str = "ServiceActor";

impl Actor for ServiceActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("ServiceActor started");
        heart_beat::notify_actor_started(NAME);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("ServiceActor stopped");
        heart_beat::notify_actor_stopped(NAME);
    }
}
