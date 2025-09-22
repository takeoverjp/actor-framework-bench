use actix::prelude::*;

pub struct PlatformActor;

use crate::heart_beat;

const NAME: &str = "PlatformActor";

impl Actor for PlatformActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("PlatformActor started");
        heart_beat::notify_actor_started(NAME);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("PlatformActor stopped");
        heart_beat::notify_actor_stopped(NAME);
    }
}
