use kiss3d::{
    camera::ArcBall,
    event::{Action, Key},
    light::Light,
    nalgebra::{Point3, Translation3},
    window::Window,
};
use ode_rs as ode;

fn main() {
    let mut win = Window::new("ode-rs cube");
    win.set_light(Light::StickToCamera);
    let mut cam = ArcBall::new(Point3::new(3.0, 3.0, 3.0), Point3::origin());

    let mut world = ode::World::new();
    world.add_floor();
    let mass = ode::Mass::cube(100.0, 1.0);
    let mut body = world.add_body(mass, 0.0, 0.0, 1.0);
    body.add_cube(1.0);

    let mut c = win.add_cube(1.0, 1.0, 1.0);
    c.set_color(1.0, 0.0, 0.0);

    while win.render_with_camera(&mut cam) {
        if win.get_key(Key::Space) == Action::Press {
            body.set_pos(0.0, 0.0, 1.0);
        }

        let (x, y, z) = body.get_pos();
        c.set_local_translation(Translation3::new(x as f32, z as f32, y as f32));
        world.step(0.01, true);
    }
}
