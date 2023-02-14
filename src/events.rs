use crate::file::File;

#[derive(Debug)]
#[allow(dead_code)]
pub enum EventFiles {
    Created(File),
    Modified(File),
    Eliminated(File),
}
