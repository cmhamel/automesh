use automesh::{Abaqus, Voxels};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    str,
};

const NELX: usize = 4;
const NELY: usize = 5;
const NELZ: usize = 3;
const NEL: [usize; 3] = [NELX, NELY, NELZ];
const SCALE: [f64; 3] = [1.0, 1.0, 1.0];
const TRANSLATE: [f64; 3] = [0.0, 0.0, 0.0];

fn read_both_lines(
    gold: &mut String,
    gold_reader: &mut BufReader<File>,
    line: &mut String,
    line_reader: &mut BufReader<File>,
) {
    gold.clear();
    line.clear();
    gold_reader.read_line(gold).unwrap();
    line_reader.read_line(line).unwrap();
}

fn read_both_files(
    gold: &mut String,
    gold_reader: &mut BufReader<File>,
    line: &mut String,
    line_reader: &mut BufReader<File>,
) {
    gold.clear();
    line.clear();
    gold_reader.read_to_string(gold).unwrap();
    line_reader.read_to_string(line).unwrap();
}

#[cfg(not(target_os = "windows"))]
mod write_inp {
    use super::*;
    #[test]
    fn letter_f_3d() {
        let voxels = Voxels::from_spn("tests/input/letter_f_3d.spn", NEL);
        let fem = voxels.into_finite_elements(&SCALE, &TRANSLATE);
        fem.write_inp("target/letter_f_3d.inp");
        let mut gold = String::new();
        let mut line = String::new();
        let mut gold_reader = BufReader::new(File::open("tests/input/letter_f_3d.inp").unwrap());
        let mut line_reader = BufReader::new(File::open("target/letter_f_3d.inp").unwrap());
        for _ in 0..2 {
            read_both_lines(&mut gold, &mut gold_reader, &mut line, &mut line_reader);
            assert_eq!(gold, line);
        }
        read_both_lines(&mut gold, &mut gold_reader, &mut line, &mut line_reader);
        let version_prefix_gold = str::from_utf8(&gold.as_bytes()[0..8]).unwrap();
        let version_prefix_line = str::from_utf8(&line.as_bytes()[0..8]).unwrap();
        assert_eq!(version_prefix_gold, version_prefix_line);
        let version_gold = env!("CARGO_PKG_VERSION");
        let version_line = str::from_utf8(&line.as_bytes()[8..13]).unwrap();
        assert_eq!(version_gold, version_line);
        read_both_lines(&mut gold, &mut gold_reader, &mut line, &mut line_reader);
        let time_prefix_gold = str::from_utf8(&gold.as_bytes()[0..17]).unwrap();
        let time_prefix_line = str::from_utf8(&line.as_bytes()[0..17]).unwrap();
        assert_eq!(time_prefix_gold, time_prefix_line);
        read_both_files(&mut gold, &mut gold_reader, &mut line, &mut line_reader);
        assert_eq!(gold, line);
    }
}
