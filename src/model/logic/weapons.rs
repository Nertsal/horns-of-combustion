use super::*;

impl Model {
    pub fn update_weapons(&mut self, delta_time: Time) {
        self.update_actors(delta_time);
    }

    fn update_actors(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(optic = "._Some")]
            gun: &'a mut Gun,
        }

        let mut query = query_actor_ref!(self.actors);
        let mut iter = query.iter_mut();
        while let Some((_id, actor)) = iter.next() {
            update_weapon(&mut actor.gun.shot_delay, delta_time);
        }
    }
}

fn update_weapon(shot_delay: &mut Time, delta_time: Time) {
    *shot_delay = (*shot_delay - delta_time).max(Time::ZERO);
}
