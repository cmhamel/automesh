// tests/io.rs

/// This module contains input/output (IO) integration tests for the `Spn`
/// functionality.
#[cfg(any(doc, test))]
mod io_tests {

    use automesh::Spn;

    // TODO: What doesn't the triple forward slash (///) and the `pub` function make
    // the documentation appear with the `cargo doc` command?

    /// Guard against case where file exists, but it cannot be read,
    /// for example, by specifying a text file, `f.txt`, which is not
    /// `.npy` file.
    #[test]
    #[should_panic(expected = "File type must be .npy")]
    pub fn from_npy_file_unreadable() {
        let _spn = Spn::from_npy("tests/input/f.txt");
    }

    /// Guard against case where file does not exist.
    /// Precondition: `f_file_nonexistent.npy` actually does not exist.
    #[test]
    #[should_panic(expected = "Could not find the .npy file")]
    fn from_npy_file_nonexistent() {
        let _spn = Spn::from_npy("tests/input/f_file_nonexistent.npy");
    }

    /// Guard against the case where the .npy file exists, but it
    /// cannot be opened. Here we have created an encrypted file
    /// contents contained `[doc/encrypted.md](../doc/encrypted.md)`.
    #[test]
    #[should_panic(expected = "Could not open the .npy file")]
    fn from_npy_file_unopenable() {
        let _spn = Spn::from_npy("tests/input/encrypted.npy");
    }
}
