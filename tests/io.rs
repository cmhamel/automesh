use automesh::Spn;

#[test]
#[should_panic(expected = "File type must be .npy")]
fn from_npy_file_unreadable() {
    // Guard against case where file exists, but it cannot be read,
    // for example, by specifying a text file, `f.txt`, which is not
    // `.npy` file.
    let _spn = Spn::from_npy("tests/input/f.txt");
}

#[test]
#[should_panic(expected = "Could not find the .npy file")]
fn from_npy_file_nonexistent() {
    // Guard against case where file does not exist.
    // Precondition: `f_file_nonexistent.npy` actually does not exist.
    let _spn = Spn::from_npy("tests/input/f_file_nonexistent.npy");
}

#[test]
#[should_panic(expected = "Could not open the .npy file")]
fn from_npy_file_unopenable() {
    // Guard against the case where the .npy file exists, but it
    // cannot be opened. Here we have created an encrypted file
    // contents contained `doc/encrypted.md`.
    let _spn = Spn::from_npy("tests/input/encrypted.npy");
}
