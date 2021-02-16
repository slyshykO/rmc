//
// author: Slyshyk Oleksiy
//

use fs_err::{File, OpenOptions};
use std::io::{Read, Write};

pub(crate) fn file_content<P>(file_name: P) -> std::io::Result<Vec<u8>>
where
    P: Into<std::path::PathBuf>,
{
    let mut s: Vec<u8> = vec![];
    File::open(file_name).and_then(|mut f| f.read_to_end(&mut s))?;
    Ok(s)
}

pub(crate) fn rewrite_file_content<P, C>(file_name: P, new_content: C) -> std::io::Result<()>
where
    P: Into<std::path::PathBuf>,
    C: AsRef<[u8]>,
{
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .and_then(|mut f| f.write_all(new_content.as_ref()))?;
    Ok(())
}
