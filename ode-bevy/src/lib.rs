use bevy::{core::FixedTimestep, prelude::*};
use ode_rs as ode;
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct ODE {
    step_size: f64,

    world: ode::World,
    bodies: HashMap<(u32, u32), ode::Body>,
}

pub struct Body {
    pub mass: f64,
}

pub struct Collider {
    pub side_len: f64,
}

fn ode_setup() {}

fn ode_add_bodies(
    mut ode: NonSendMut<ODE>,
    query: Query<(Entity, &Body, &Transform), Added<Body>>,
) {
    for (e, body, transform) in query.iter() {
        println!("added body ({},{}), linking", e.id(), e.generation());
        let mass = ode::Mass::cube(body.mass, 1.0);
        let body = ode.world.add_body(
            mass,
            transform.translation.x as f64,
            transform.translation.y as f64,
            transform.translation.z as f64,
        );
        ode.bodies.insert((e.id(), e.generation()), body);
    }
}

fn ode_add_colliders(
    mut ode: NonSendMut<ODE>,
    query: Query<(Entity, &Collider, &Transform, &Parent), Added<Collider>>,
) {
    for (e, collider, transform, parent) in query.iter() {
        println!("added collider ({},{}), linking", e.id(), e.generation());
        let body = ode
            .bodies
            .get_mut(&(parent.id(), parent.generation()))
            .unwrap();
        body.add_cube(collider.side_len);
    }
}

fn ode_remove_bodies(world: &mut World) {
    let removed_colliders: Vec<Entity> = world.removed::<Collider>().collect();
    let removed_bodies: Vec<Entity> = world.removed::<Body>().collect();

    let mut ode = world.get_non_send_resource_mut::<ODE>().unwrap();
    for e in removed_bodies {
        ode.bodies.remove(&(e.id(), e.generation()));
    }
}

fn ode_update_transforms(
    ode: NonSend<ODE>,
    mut query: Query<(Entity, &mut Transform), With<Body>>,
) {
    for (e, mut transform) in query.iter_mut() {
        if let Some(body) = ode.bodies.get(&(e.id(), e.generation())) {
            let (x, y, z) = body.get_pos();
            transform.translation = Vec3::new(x as f32, y as f32, z as f32);
            let (w, x, y, z) = body.get_quat();
            transform.rotation = Quat::from_xyzw(x as f32, y as f32, z as f32, w as f32);
        }
    }
}

pub struct StepTime(pub f32);

fn ode_step(mut ode: NonSendMut<ODE>, mut step_time: ResMut<StepTime>) {
    let t0 = Instant::now();
    let step_size = ode.step_size;
    ode.world.step(step_size, true);
    step_time.0 = t0.elapsed().as_secs_f32();
}

pub struct ODEPlugin {
    pub hz: f64,
}
impl Plugin for ODEPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let mut world = ode::World::new();
        world.add_floor();

        let step_size = 1.0 / self.hz;

        app.insert_non_send_resource(ODE {
            step_size,
            world,
            bodies: HashMap::new(),
        })
        .insert_resource(StepTime(1.0))
        .add_startup_system(ode_setup.system())
        .add_system(ode_add_bodies.system().label("add_bodies"))
        .add_system(ode_add_colliders.system().after("add_bodies"))
        .add_system_to_stage(CoreStage::PostUpdate, ode_remove_bodies.exclusive_system())
        .add_system(ode_update_transforms.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(step_size))
                .with_system(ode_step.system()),
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
