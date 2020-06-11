//
// author: Slyshyk Oleksiy
//

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub(crate) fn file_content<T: AsRef<std::path::Path>>(file_name: T) -> std::io::Result<Vec<u8>> {
    let mut s: Vec<u8> = vec![];
    File::open(file_name).and_then(|mut f| f.read_to_end(&mut s))?;
    Ok(s)
}

pub(crate) fn rewrite_file_content<P, C>(file_name: P, new_content: C) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
    C: AsRef<[u8]>,
{
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)
        .and_then(|mut f| f.write_all(new_content.as_ref()))?;
    Ok(())
}
