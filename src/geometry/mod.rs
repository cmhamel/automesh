// generic stuff used in all prmitives
// use bbox;
use enum_dispatch::enum_dispatch;
use min_max::{max, min};
use nalgebra::{self, Vector3};
use std::fs::File;
use std::io::{BufWriter, Write};

// some local types
// pub type BoundingBox = bbox::BoundingBox<f64>;
pub type Point = nalgebra::Point3<f64>;
pub type Matrix = nalgebra::Matrix3<f64>;
pub type Vector = nalgebra::Vector3<f64>;
// pub type Widths = nalgebra::Point3<f64>;

pub struct BoundingBox {
    min: [f64; 3],
    max: [f64; 3]
}

impl BoundingBox {
    pub fn new(min: [f64; 3], max: [f64; 3]) -> Self {
        Self {
            min: min,
            max: max
        }
    }

    pub fn corners(&self) -> [Vector; 8] {
        let vec: [Vector; 8] = [
            Vector::new(self.min[0], self.min[1], self.min[2]),
            Vector::new(self.max[0], self.min[1], self.min[2]),
            Vector::new(self.max[0], self.max[1], self.min[2]),
            Vector::new(self.min[0], self.max[1], self.min[2]),
            //
            Vector::new(self.min[0], self.min[1], self.max[2]),
            Vector::new(self.max[0], self.min[1], self.max[2]),
            Vector::new(self.max[0], self.max[1], self.max[2]),
            Vector::new(self.min[0], self.max[1], self.max[2])
        ];
        vec
    }

    pub fn intersect(&self, bb: BoundingBox) -> BoundingBox {
        let bb_new = BoundingBox::new(
            [
                max(self.min[0], bb.min[0]),
                max(self.min[1], bb.min[1]),
                max(self.min[2], bb.min[2])
            ],
            [
                min(self.max[0], bb.max[0]),
                min(self.max[1], bb.max[1]),
                min(self.max[2], bb.max[2])
            ]
        );
        bb_new
    }

    pub fn union(&self, bb: BoundingBox) -> BoundingBox {
        let bb_new = BoundingBox::new(
            [
                min(self.min[0], bb.min[0]),
                min(self.min[1], bb.min[1]),
                min(self.min[2], bb.min[2])
            ],
            [
                max(self.max[0], bb.max[0]),
                max(self.max[1], bb.max[1]),
                max(self.max[2], bb.max[2])
            ]
        );
        bb_new
    }
}

/// enum of all currently supported primitives
#[enum_dispatch(GeometricPrimitive)]
// #[pyclass]
#[derive(Clone, Debug)]
pub enum Primitive {
    // affine transformations
    Rotation(Rotation),
    Translation(Translation),
    // booleans
    Difference(Difference),
    Intersection(Intersection),
    Union(Union),
    // 2D Primitives
    // Circle(Circle),
    // Rectangle(Rectangle),
    // 3D Primitives
    // Ellipsoid(Ellipsoid),
    Sphere(Sphere),
    Torus(Torus)
}

/// base trait for all Primitive
#[enum_dispatch]
pub trait GeometricPrimitive {
    /// method that returns a bounding box using
    /// the crate bbox
    fn bounding_box(&self) -> BoundingBox;
    /// method that takes in a point and
    /// returns the evaluated signed distance
    /// function for the geometry
    fn sdf(&self, v: &Point) -> f64;

    /// naively writes all geometries as a single material
    /// would have to merge seperate geometries somehow
    fn to_spn(&self, file_path: &str, nx: usize, ny: usize, nz: usize) -> () {
        let bb = self.bounding_box();
        let mut voxels = Vec::<usize>::new();
        let min_x = bb.min[0];
        let min_y = bb.min[1];
        let min_z = bb.min[2];
        let max_x = bb.max[0];
        let max_y = bb.max[1];
        let max_z = bb.max[2];

        let voxel_size_x = (max_x - min_x) / nx as f64;
        let voxel_size_y = (max_y - min_y) / ny as f64;
        let voxel_size_z = (max_z - min_z) / nz as f64;

        for xi in 0..nx {
            for yi in 0..ny {
                for zi in 0..nz {
                    // form a vector v at teh centroid of the voxel
                    // let x_centroid = (xi as f64 + 0.5) * ;
                    let x_centroid = min_x + (xi as f64 + 0.5) * voxel_size_x;
                    let y_centroid = min_y + (yi as f64 + 0.5) * voxel_size_y;
                    let z_centroid = min_z + (zi as f64 + 0.5) * voxel_size_z;
                    let v = Point::new(x_centroid, y_centroid, z_centroid);
                    if self.sdf(&v) <= 0. {
                        println!("Got a voxel");
                        voxels.push(1);
                    } else {
                        voxels.push(0);
                    }
                }
            }
        }

        let mut file = BufWriter::new(File::create(file_path).unwrap());
        for voxel in voxels {
            let _ = writeln!(file, "{}", voxel.to_string()).unwrap();
        }
        // let _ = voxels
        //     .iter()
        //     .map(|entry| writeln!(file, "{}", entry));
    }
}

#[derive(Clone, Debug)]
pub struct AffineTransformation {
    a: Matrix,
    c: Vector,
    primitive: Box<Primitive>
}

impl AffineTransformation {
    pub fn new(
        a: Matrix,
        c: Vector,
        primitive: Primitive
    ) -> Self {
        Self {
            a: a,
            // ainv: a.try_inverse().expect("Failed to invert rotation matrix"),
            c: c,
            primitive: Box::new(primitive)
        }
    }

    pub fn transform(&self, x: &Vector) -> Vector {
        // self.a.dot(x)
        self.a * x + self.c
    }

    pub fn inverse_transform(&self) -> AffineTransformation {
        // -self.ainv * x - self.c
        let ainv = self.a.try_inverse().expect("Failed to invert rotation matrix");
        let cinv = -ainv * self.c;
        AffineTransformation {
            a: ainv,
            c: cinv,
            primitive: self.primitive.clone()
        }
    }
}

// #[pyclass]
// #[derive(Clone, Debug, Deserialize, Serialize)]
/// struct that holds onto a primitive (can be anything in enum Primitive)
/// and a rotation matrix from nalgebra
#[derive(Clone, Debug)]
pub struct Rotation {
    rotation: nalgebra::Rotation3<f64>,
    primitive: Box<Primitive>
}

impl Rotation {
    pub fn new(primitive: Primitive, axis: &nalgebra::Unit<Vector3<f64>>, angle: f64) -> Self {
        Rotation {
            rotation: nalgebra::Rotation3::from_axis_angle(axis, angle),
            primitive: Box::new(primitive)
        }
    }
}

impl GeometricPrimitive for Rotation {
    fn bounding_box(&self) -> BoundingBox {
        self.primitive.bounding_box().transform(&self.rotation.to_homogeneous())
    }

    fn sdf(&self, v: &Point) -> f64 {
        let v_temp = self.rotation.inverse().transform_point(v);
        self.primitive.sdf(&v_temp)
    }
}

pub fn rotate(primitive: Primitive, axis: char, angle: f64) -> Primitive {
    let axis = match axis {
        'x' => Vector3::x_axis(),
        'y' => Vector3::y_axis(),
        'z' => Vector3::z_axis(),
        _   => panic!("Undefined axis")
    };
    Primitive::Rotation(Rotation::new(primitive, &axis, angle))
}

// #[pyclass]
// #[derive(Clone, Debug, Deserialize, Serialize)]
#[derive(Clone, Debug)]
pub struct Translation {
    translation: nalgebra::Translation3<f64>,
    primitive: Box<Primitive>
}

impl Translation {
    pub fn new(primitive: Primitive, x: f64, y: f64, z: f64) -> Self {
        Translation {
            translation: nalgebra::Translation3::new(x, y, z),
            primitive: Box::new(primitive)
        }
    }
}

impl GeometricPrimitive for Translation {
    fn bounding_box(&self) -> BoundingBox {
        self.primitive.bounding_box().transform(&self.translation.to_homogeneous())
    }

    fn sdf(&self, v: &Point) -> f64 {
        let v_temp = self.translation.inverse().transform_point(v);
        self.primitive.sdf(&v_temp)
    }
}

pub fn translate(primitive: Primitive, x: f64, y: f64, z: f64) -> Primitive {
    Primitive::Translation(Translation::new(primitive, x, y, z))
}

// #[pyclass]
// #[derive(Clone, Debug, Deserialize, Serialize)]
#[derive(Clone, Debug)]
pub struct Difference {
    left: Box<Primitive>,
    right: Box<Primitive>
}

impl Difference {
    pub fn new(left: Primitive, right: Primitive) -> Self {
        Difference {
            left: Box::new(left),
            right: Box::new(right)
        }
    }
}

impl GeometricPrimitive for Difference {
    fn bounding_box(&self) -> BoundingBox {
        self.left.bounding_box()
    }

    fn sdf(&self, v: &Point) -> f64 {
        max(self.left.sdf(v), -self.right.sdf(v))
    }
}

// #[pyclass]
// #[derive(Clone, Debug, Deserialize, Serialize)]
#[derive(Clone, Debug)]
pub struct Intersection {
    left: Box<Primitive>,
    right: Box<Primitive>
}

impl Intersection {
    pub fn new(left: Primitive, right: Primitive) -> Self {
        Intersection {
            left: Box::new(left),
            right: Box::new(right)
        }
    }
}

impl GeometricPrimitive for Intersection {
    fn bounding_box(&self) -> BoundingBox {
        self.left.bounding_box().intersection(&self.right.bounding_box())
    }

    fn sdf(&self, v: &Point) -> f64 {
        max(self.left.sdf(v), self.right.sdf(v))
    }
}


// #[pyclass]
// #[derive(Clone, Debug, Deserialize, Serialize)]
#[derive(Clone, Debug)]
pub struct Union {
    left: Box<Primitive>,
    right: Box<Primitive>
}

impl Union {
    pub fn new(left: Primitive, right: Primitive) -> Self {
        Union {
            left: Box::new(left),
            right: Box::new(right)
        }
    }
}

impl GeometricPrimitive for Union {
    fn bounding_box(&self) -> BoundingBox {
        let bb = self.left.bounding_box().union(&self.right.bounding_box());
        bb
    }

    fn sdf(&self, v: &Point) -> f64 {
        min(self.left.sdf(v), self.right.sdf(v))
    }
}

// actual shapes below
// #[pyclass]
// #[derive(Clone, Debug, Deserialize, Serialize)]
#[derive(Clone, Debug)]
pub struct Sphere {
    radius: f64
}

impl Sphere {
    pub fn new(radius: f64) -> Self {
        Sphere {
            radius: radius
        }
    }
}

impl GeometricPrimitive for Sphere {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            &Point::new(-self.radius, -self.radius, -self.radius),
            &Point::new(self.radius, self.radius, self.radius)
        )
    }

    fn sdf(&self, v: &Point) -> f64 {
        v.coords.norm() - self.radius
    }
}

// #[pyclass]
// #[derive(Clone, Debug, Deserialize, Serialize)]
#[derive(Clone, Debug)]
pub struct Torus {
  a: f64,
  c: f64
}

impl Torus {
  pub fn new(a: f64, c: f64) -> Self {
    Torus {
      a: a,
      c: c
    }
  }
}

impl GeometricPrimitive for Torus {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            &Point::new(-self.c - self.a, -self.c - self.a, -self.a),
            &Point::new(self.c + self.a, self.c + self.a, self.a)
        )
    }

    fn sdf(&self, v: &Point) -> f64 {
        // julia implementation below...
        // can't quite get this to work since rust is so 
        // unreadable for numerical stuff...
        // it's almost correct.
        // a, c = g.a, g.c
        // return sqrt((c - sqrt(v[1]^2 + v[2]^2))^2 + v[3]^2) - a^2
        // let (a, c) = self.a, self.c;
        let a = self.a;
        let c = self.c;

        ((c - (v.x * v.x + v.y * v.y).sqrt()) * (c - (v.x * v.x + v.y * v.y).sqrt()) + v.z * v.z).sqrt() - a * a

    }
}
