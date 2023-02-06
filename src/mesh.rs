use std::fmt::Display;

use rrplug::wrappers::vector::Vector3;

#[derive(Debug)]
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

#[derive(Debug)]
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

pub fn mesh_to_brush(point1: Vector3, point2: Vector3, texture: String) -> Mesh {
    let point1 = Vector3::from([point1.x.round(), point1.y.round(), point1.z.round()]);
    let point2 = Vector3::from([point2.x.round(), point2.y.round(), point2.z.round()]);

    let min_x = point1.x.min(point2.x);
    let max_x = point1.x.max(point2.x);
    let min_y = point1.y.min(point2.y);
    let max_y = point1.y.max(point2.y);
    let min_z = point1.z.min(point2.z);
    let max_z = point1.z.max(point2.z);

    // 0 should be replaced with mins instead

    // ( min_x max_y max_z ) ( min_x 0 max_z ) ( min_x 0 0 )
    let up = Face {
        topconer: (min_x, max_y, max_z).into(),
        anyconrer: (min_x, min_y, max_z).into(),
        bottomcorner: (min_x, min_y, min_z).into(),
        texture: texture.clone(),
    };

    // ( 0 min_y max_z ) ( max_x min_y max_z ) ( max_x min_y 0 )
    let down = Face {
        topconer: (min_x, min_y, max_z).into(),
        anyconrer: (max_x, min_y, max_z).into(),
        bottomcorner: (max_x, min_y, min_z).into(),
        texture: texture.clone(),
    };

    // ( max_x 0 0 ) ( max_x max_y 0 ) ( 0 max_y 0 )
    let forward = Face {
        topconer: (max_x, min_y, min_z).into(),
        anyconrer: (max_x, max_y, min_z).into(),
        bottomcorner: (min_x, max_y, min_z).into(),
        texture: texture.clone(),
    };

    // ( 0 max_y max_z ) ( max_x max_y max_z ) ( max_x 0 max_z )
    let backwards = Face {
        topconer: (min_x, max_y, max_z).into(),
        anyconrer: (max_x, max_y, max_z).into(),
        bottomcorner: (max_x, min_y, max_z).into(),
        texture: texture.clone(),
    };

    // ( max_x max_y 0 ) ( max_x max_y max_z ) ( 0 max_y max_z )
    let right = Face {
        topconer: (max_x, max_y, min_z).into(),
        anyconrer: (max_x, max_y, max_z).into(),
        bottomcorner: (min_x, max_y, max_z).into(),
        texture: texture.clone(),
    };

    // ( max_x 0 max_z ) ( max_x max_y max_z ) ( max_x max_y 0 )
    let left = Face {
        topconer: (max_x, min_y, max_z).into(),
        anyconrer: (max_x, max_y, max_z).into(),
        bottomcorner: (max_x, max_y, min_z).into(),
        texture,
    };

    Mesh {
        up,
        down,
        left,
        right,
        forward,
        backwards,
    }
}
