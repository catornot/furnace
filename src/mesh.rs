use std::fmt::Display;

use rrplug::{prelude::*, sq_return_null, sqfunction, wrappers::vector::Vector3};

use crate::FURNACE;

pub struct Face {
    pub topconer: Vector3,
    pub anyconrer: Vector3,
    pub bottomcorner: Vector3,
    pub texture: String,
}

impl Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) ( ( 0.0078125 0 -0 ) ( -0 0.0078125 0 ) ) {} 0 0 0",
            self.topconer.x,
            self.topconer.y,
            self.topconer.z,
            self.anyconrer.x,
            self.anyconrer.y,
            self.anyconrer.z,
            self.bottomcorner.x,
            self.bottomcorner.y,
            self.bottomcorner.z,
            self.texture
        ))
    }
}

pub struct Mesh {
    pub up: Face,
    pub down: Face,
    pub left: Face,
    pub right: Face,
    pub forward: Face,
    pub backwards: Face,
}

impl Display for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            r#"{{
brushDef
{{
{}
{}
{}
{}
{}
{}
}}
}}"#,
            self.up, self.down, self.left, self.right, self.forward, self.backwards
        ))
    }
}

pub fn mesh_register_sqfunction(plugin_data: &PluginData) {
    _ = plugin_data.register_sq_functions(info_push_mesh);
}

#[sqfunction(VM=SERVER,ExportName=PushMesh)]
pub fn push_mesh(point1: Vector3, point2: Vector3) {

    let point1 = Vector3::from( [point1.x.round(),point1.y.round(),point1.z.round()] );
    let point2 = Vector3::from( [point2.x.round(),point2.y.round(),point2.z.round()] );

    let z = point1.z.max(point2.z);

    let up = Face {
        topconer: (point1.x, point1.y, z).into(),
        anyconrer: (point1.x, point2.y, z).into(),
        bottomcorner: (point2.x, point2.y, z).into(),
        texture: "world/dev/dev_white_512".into(),
    };

    let z = point1.z.min(point2.z);

    let down = Face {
        topconer: (point1.x, point1.y, z).into(),
        anyconrer: (point1.x, point2.y, z).into(),
        bottomcorner: (point2.x, point2.y, z).into(),
        texture: "world/dev/dev_white_512".into(),
    };

    let x = point1.x.max(point2.x);

    let forward = Face {
        topconer: (x, point1.y, point1.z).into(),
        anyconrer: (x, point2.y, point1.z).into(),
        bottomcorner: (x, point2.y, point2.z).into(),
        texture: "world/dev/dev_white_512".into(),
    };

    let x = point1.x.min(point2.x);

    let backwards = Face {
        topconer: (x, point1.y, point1.z).into(),
        anyconrer: (x, point2.y, point1.z).into(),
        bottomcorner: (x, point2.y, point2.z).into(),
        texture: "world/dev/dev_white_512".into(),
    };

    let y = point1.y.max(point2.y);

    let right = Face {
        topconer: (point1.x, y, point1.z).into(),
        anyconrer: (point2.x, y, point1.z).into(),
        bottomcorner: (point2.x, y, point2.z).into(),
        texture: "world/dev/dev_white_512".into(),
    };

    let y = point1.y.min(point2.y);

    let left = Face {
        topconer: (point1.x, y, point1.z).into(),
        anyconrer: (point2.x, y, point1.z).into(),
        bottomcorner: (point2.x, y, point2.z).into(),
        texture: "world/dev/dev_white_512".into(),
    };

    let mut furnace = FURNACE.wait().lock().unwrap();

    furnace.meshes.push(Mesh {
        up,
        down,
        left,
        right,
        forward,
        backwards,
    });

    sq_return_null!()
}
