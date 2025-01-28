// use automesh::{Tessellation, NSD};
use automesh::Tessellation;

mod from_stl {
    use super::*;
    #[test]
    #[cfg(not(target_os = "windows"))]
    #[should_panic(expected = "No such file or directory")]
    fn file_nonexistent() {
        Tessellation::from_stl("tests/input/f_file_nonexistent.stl");
    }
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn file_one_facet() {
        let tess = Tessellation::from_stl("tests/input/one_facet.stl");
        println!("{}", tess);
    }
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn file_single() {
        let tess = Tessellation::from_stl("tests/input/single.stl");
        println!("{}", tess);
    }
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn file_double() {
        let tess = Tessellation::from_stl("tests/input/double.stl");
        println!("{}", tess);
    }

}


mod write_stl {
    use super::*;
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn one_facet_read_write() {
        let tess = Tessellation::from_stl("tests/input/one_facet.stl");
        println!("{}", tess);
    }
}