use std::path::Path;
use std::env;
use std::io;

const TRASH_PATH: &str = "/home/theo/test";


fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let path_arg = &args[1];
    let path = Path::new(path_arg);

    match path.try_exists() {
        Ok(false) => {
            return Err(io::Error::new(io::ErrorKind::NotFound ,"The file or folder dosen't exist"));
        },
        Err(error) => {
            return Err(error);
        },
        _ => (),
    }

    let name = path
        .file_name().ok_or(io::Error::other("Can't get the file or folder name"))?
        .to_str().ok_or(io::Error::other("Can't convert OS String"))?;

    let time = chrono::Local::now();

    let target_str = format!("{}/{}-{}", TRASH_PATH, name, time.format("%d-%m-%H:%M:%S"));
    let target_path = Path::new(&target_str);

    std::fs::create_dir_all(target_path.parent().ok_or(io::Error::other("Couldn't get parent of path"))?)?;
    std::fs::rename(path, target_path)?;

    println!("{:?} -> {:?}", path, &target_path);
    Ok(())
}
