use automesh::{
    GeometricPrimitive, Primitive,
    Sphere, Torus,
    rotate, translate,
    Difference, Intersection, Union
};
use bbox;
// use nalgebra::{self, Vector3}

pub type BoundingBox = bbox::BoundingBox<f64>;
pub type Point = nalgebra::Point3<f64>;
pub type Widths = nalgebra::Point3<f64>;

mod test_sphere {
    use super::*;
  
    #[test]
    fn test_bounding_box() {
        let c = Sphere::new(1.);
        let bb = c.bounding_box();
        assert_eq!(bb.min, Point::new(-1., -1., -1.));
        assert_eq!(bb.max, Widths::new(1., 1., 1.));

        let c = Sphere::new(5.);
        let bb = c.bounding_box();
        assert_eq!(bb.min, Point::new(-5., -5., -5.));
        assert_eq!(bb.max, Widths::new(5., 5., 5.));
    }
    
    #[test]
    fn test_inside_sphere() {
        let c = Sphere::new(1.);
        let v = Point::new(0., 0., 0.);
        assert!(c.sdf(&v) < 0.);

        let c = Sphere::new(5.);
        let v = Point::new(0., 0., 0.);
        assert!(c.sdf(&v) < 0.);
    }
  
    #[test]
    fn test_on_boundary() {
        let c = Sphere::new(1.);
        let v = Point::new(1., 0., 0.);
        assert_eq!(c.sdf(&v), 0.);

        let c = Sphere::new(5.);
        println!("c = {:?}", c);
        let v = Point::new(5., 0., 0.);
        assert_eq!(c.sdf(&v), 0.);
    }
  
    #[test]
    fn test_outside_sphere() {
        let c = Sphere::new(1.);
        let v = Point::new(1., 1., 0.);
        assert!(c.sdf(&v) > 0.);

        let c = Sphere::new(5.);
        let v = Point::new(5., 5., 0.);
        assert!(c.sdf(&v) > 0.);
    }

    #[test]
    fn test_to_spn() {
        let c = Sphere::new(1.);
        c.to_spn("test_sphere.spn", 40, 40, 40);
    }
}

mod test_translation {
    use super::*;
  
    #[test]
    fn test_bounding_box_no_translation() {
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 0., 0., 0.);
        let bb = t.bounding_box();
        assert_eq!(bb.min, Point::new(-1., -1., -1.));
        assert_eq!(bb.max, Point::new(1., 1., 1.));
    }
  
    #[test]
    fn test_bounding_box_x_translation() {
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 1., 0., 0.);
        let bb = t.bounding_box();
        assert_eq!(bb.min, Point::new(0., -1., -1.));
        assert_eq!(bb.max, Point::new(2., 1., 1.));
    }
  
    #[test]
    fn test_bounding_box_y_translation() {
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 0., 1., 0.);
        let bb = t.bounding_box();
        assert_eq!(bb.min, Point::new(-1., 0., -1.));
        assert_eq!(bb.max, Point::new(1., 2., 1.));
    }
  
    #[test]
    fn test_bounding_box_z_translation() {
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 0., 0., 1.);
        let bb = t.bounding_box();
        assert_eq!(bb.min, Point::new(-1., -1., 0.));
        assert_eq!(bb.max, Point::new(1., 1., 2.));
    }
  
    #[test]
    fn test_inside_translated_sphere() {
        // no translation
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 0., 0., 0.);
        let v = Point::new(0., 0., 0.);
        assert!(t.sdf(&v) < 0.);

        // x translation
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 1., 0., 0.);
        let v = Point::new(1., 0., 0.);
        assert!(t.sdf(&v) < 0.);

        // y translation
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 0., 1., 0.);
        let v = Point::new(0., 1., 0.);
        assert!(t.sdf(&v) < 0.);

        // z translation
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 0., 0., 1.);
        let v = Point::new(0., 0., 1.);
        assert!(t.sdf(&v) < 0.);
    }
  
    #[test]
    fn test_on_translated_boundary() {
        // no translation
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 0., 0., 0.);
        let v = Point::new(1., 0., 0.);
        assert_eq!(t.sdf(&v), 0.);

        // x translation
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 1., 0., 0.);
        let v = Point::new(2., 0., 0.);
        assert_eq!(t.sdf(&v), 0.);

        // y translation
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 0., 1., 0.);
        let v = Point::new(0., 2., 0.);
        assert_eq!(t.sdf(&v), 0.);

        // z translation
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 0., 0., 1.);
        let v = Point::new(1., 0., 1.);
        assert_eq!(t.sdf(&v), 0.);
    }
  
    #[test]
    fn test_outside_translated_sphere() {
        // no translation
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 0., 0., 0.);
        let v = Point::new(2., 0., 0.);
        assert!(t.sdf(&v) > 0.);

        // x translation
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 1., 0., 0.);
        let v = Point::new(2., 1., 0.);
        assert!(t.sdf(&v) > 0.);

        // y translation
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 0., 1., 0.);
        let v = Point::new(1., 2., 0.);
        assert!(t.sdf(&v) > 0.);

        // z translation
        let c: Primitive = Sphere::new(1.).into();
        let t = translate(c, 0., 0., 1.);
        let v = Point::new(1., 1., 1.);
        assert!(t.sdf(&v) > 0.);
    }
}

mod test_complicated {
    use super::*;

    #[test]
    fn test_write_spn() {
        let s1 = Sphere::new(1.);
        let s2 = Sphere::new(1.5);
        let s3 = translate(Primitive::Sphere(s2), 1., 1., 0.);
        let g = Union::new(Primitive::Sphere(s1), s3);
        // let g = Torus::new(1., 2.5);
        g.to_spn("test_complicated.spn", 250, 250, 250);
    }
}