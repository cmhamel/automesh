use std::
{
    fs::File,
    io::
    {
        BufRead,
        BufReader
    }
};

type Data = Vec<u8>;

pub struct Spn
{
    data: Data,
    nelx: usize,
    nely: usize,
    nelz: usize
}

impl Spn
{
    pub fn compute_mesh(&self)
    {
        todo!()
    }
    fn get_data(&self) -> &Data
    {
        &self.data
    }
    pub fn new(file_path: &str, nelx: usize, nely: usize, nelz: usize) -> Self
    {
        let data = BufReader::new(
            File::open(file_path).expect("File was not found.")
        ).lines().map(|line|
            line.unwrap().parse().unwrap()
        ).collect();
        Self
        {
            data,
            nelx,
            nely,
            nelz
        }
    }
}
