
#[derive(Debug)]
pub struct File {
    pub name: String,
    pub data: Vec<u8>,
}

pub fn open(f: &mut File) -> bool {
    true
}

pub fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
pub fn read(f: &mut File, save_to: &mut Vec<u8>) {
    unimplemented!()
}