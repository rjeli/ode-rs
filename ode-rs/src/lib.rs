use ode_sys as ode;
use std::convert::TryInto;
use std::rc::Rc;

struct CollisionData {
    world: ode::dWorldID,
    contact_group: ode::dJointGroupID,
}

unsafe extern "C" fn near_callback(
    data: *mut std::os::raw::c_void,
    o1: ode::dGeomID,
    o2: ode::dGeomID,
) {
    let collision_data = data as *const CollisionData;

    let b1 = ode::dGeomGetBody(o1);
    let b2 = ode::dGeomGetBody(o2);

    let mut contacts: [ode::dContact; 4] = std::mem::zeroed();
    let nc = ode::dCollide(
        o1,
        o2,
        4,
        &mut contacts[0].geom as *mut _,
        std::mem::size_of::<ode::dContact>().try_into().unwrap(),
    ) as usize;
    for i in 0..nc {
        contacts[i].surface.mode = ode::dContactApprox1 as _;
        contacts[i].surface.mu = 2.0;
        let c = ode::dJointCreateContact(
            (*collision_data).world,
            (*collision_data).contact_group,
            &contacts[i],
        );
        ode::dJointAttach(c, b1, b2);
    }
}

pub struct World(Rc<SharedWorld>);

struct SharedWorld {
    id: ode::dWorldID,
    cg: ode::dJointGroupID,
    space: ode::dSpaceID,
}

impl World {
    pub fn new() -> World {
        let id = unsafe {
            ode::dInitODE();
            let id = ode::dWorldCreate();
            ode::dWorldSetGravity(id, 0.0, -9.81, 0.0);
            id
        };
        World(Rc::new(SharedWorld {
            id,
            cg: unsafe { ode::dJointGroupCreate(0) },
            // space: unsafe { ode::dSimpleSpaceCreate(std::ptr::null_mut()) },
            space: unsafe {
                // let center = ode::dVector3;
                let mut center = [0.0, 0.0, 0.0, 0.0];
                let mut extents = [1000.0, 1000.0, 1000.0, 0.0];
                ode::dQuadTreeSpaceCreate(
                    std::ptr::null_mut(),
                    center.as_mut_ptr(),
                    extents.as_mut_ptr(),
                    4,
                )
            },
        }))
    }
    pub fn add_body(&mut self, m: Mass, x: f64, y: f64, z: f64) -> Body {
        unsafe {
            let b = ode::dBodyCreate(self.0.id);
            ode::dBodySetMass(b, &m.m);
            ode::dBodySetPosition(b, x, y, z);
            // ode::dBodySetQuaternion(b, q.as_mut_ptr());
            Body {
                id: b,
                world: self.0.clone(),
                geoms: Vec::new(),
            }
        }
    }
    pub fn add_floor(&mut self) {
        unsafe {
            ode::dCreatePlane(self.0.space, 0.0, 1.0, 0.0, 0.0);
        }
    }
    pub fn step(&mut self, step_size: f64, quick: bool) {
        unsafe {
            let collision_data = Box::new(CollisionData {
                world: self.0.id,
                contact_group: self.0.cg,
            });
            let data_ptr = Box::into_raw(collision_data);
            ode::dSpaceCollide(self.0.space, data_ptr as *mut _, Some(near_callback));
            Box::from_raw(data_ptr);

            if quick {
                ode::dWorldQuickStep(self.0.id, step_size);
            } else {
                ode::dWorldStep(self.0.id, step_size);
            }

            ode::dJointGroupEmpty(self.0.cg);
        }
    }
}
impl Drop for SharedWorld {
    fn drop(&mut self) {
        unsafe {
            ode::dSpaceDestroy(self.space);
            ode::dJointGroupDestroy(self.cg);
            ode::dWorldDestroy(self.id);
            ode::dCloseODE();
        }
    }
}

pub struct Mass {
    m: ode::dMass,
}
impl Mass {
    pub fn cube(total_mass: f64, side_len: f64) -> Mass {
        Mass {
            m: unsafe {
                let mut m: ode::dMass = std::mem::zeroed();
                ode::dMassSetBoxTotal(&mut m, total_mass, side_len, side_len, side_len);
                m
            },
        }
    }
}

pub struct Body {
    id: ode::dBodyID,
    world: Rc<SharedWorld>,
    geoms: Vec<Geom>,
}
impl Body {
    pub fn add_cube(&mut self, side_len: f64) -> &Geom {
        let id = unsafe {
            let id = ode::dCreateBox(self.world.space, side_len, side_len, side_len);
            ode::dGeomSetBody(id, self.id);
            id
        };
        let g = Geom { id };
        self.geoms.push(g);
        return self.geoms.last().unwrap();
    }
    pub fn set_pos(&mut self, x: f64, y: f64, z: f64) {
        unsafe {
            ode::dBodySetPosition(self.id, x, y, z);
        }
    }
    pub fn get_pos(&self) -> (f64, f64, f64) {
        unsafe {
            let pos = ode::dBodyGetPosition(self.id);
            (*pos.offset(0), *pos.offset(1), *pos.offset(2))
        }
    }
    pub fn get_quat(&self) -> (f64, f64, f64, f64) {
        unsafe {
            let quat = ode::dBodyGetQuaternion(self.id);
            (
                *quat.offset(0),
                *quat.offset(1),
                *quat.offset(2),
                *quat.offset(3),
            )
        }
    }
}
impl Drop for Body {
    fn drop(&mut self) {
        unsafe {
            println!("body drop");
            ode::dBodyDestroy(self.id);
        }
    }
}

pub struct Geom {
    id: ode::dGeomID,
}
impl Drop for Geom {
    fn drop(&mut self) {
        unsafe {
            ode::dGeomDestroy(self.id);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
