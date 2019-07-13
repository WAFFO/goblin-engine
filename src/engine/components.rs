
use specs::{Component, VecStorage};
use glm::{Vec3, Vec4};

use engine::mesh_manager::UUID;


// components
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

pub struct Velocity {
    pub position: Vec3,
    pub rotation: Vec3,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

// TODO: impl std::ops::Add for Transform + Velocity

#[derive(Default)]
pub struct StaticMesh {
    pub mesh_id: UUID,
}

impl Component for StaticMesh {
    type Storage = VecStorage<Self>;
}


pub struct Camera {
    pub rotation: Vec3,
    // pub target: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub pole_arm: f32,
}

//   side view    |    top view
//      c         | t<----z cos(yaw)
//     /|y sin(p) |   \  |
//    / |         |    \ |
//   /  |         |     \|x sin(yaw)
// t<---|xz cos(p)|      c

impl Camera {
    pub fn update(&mut self) {
        self.rotation[0] = self.pitch.cos() * self.yaw.sin() * self.pole_arm;
        self.rotation[1] = self.pitch.sin() * self.pole_arm;
        self.rotation[2] = self.pitch.cos() * self.yaw.cos() * self.pole_arm;
    }
    pub fn forward(&self) -> Vec3 {
        self.rotation.normalize()
    }
    pub fn right(&self) -> Vec3 {
        self.rotation.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize()
    }
}

impl Component for Camera {
    type Storage = VecStorage<Self>;
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            rotation: Vec3::new(0.0, 0.0, 0.0),
            // target: Vec3::new(0.0, 0.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
            pole_arm: 0.1,
        }
    }
}


// entity types
#[derive(Default)]
pub struct Solid;
impl Component for Solid {
    type Storage = VecStorage<Self>;
}

pub struct Light{
    pub color: Vec4,
}
impl Component for Light {
    type Storage = VecStorage<Self>;
}
impl Default for Light {
    fn default() -> Light {
        Light {
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

#[derive(Default)]
pub struct PlayerController;

impl Component for PlayerController {
    type Storage = VecStorage<Self>;
}