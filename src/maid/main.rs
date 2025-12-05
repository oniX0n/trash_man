use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::env;
use std::io;
use std::fs::File;
use std::path;
use std::path::PathBuf;

use chrono::DateTime;
use chrono::Local;

const TRASH_PATH: &str = "/home/theo/test/files";
const TRASH_INFO_PATH: &str = "/home/theo/test/info";


fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    run(&args[1])

    // if args.len() < 2 {
    //     eprintln!("Usage: maid FILE...");
    // }

    // for i in 1..args.len() {
    //     let arg = &args[i];
    //     if let Err(error) = run(arg) { eprintln!("Error: {}", error) }
    // }

}

fn run(arg: &String) -> io::Result<()> {

    let path_arg = arg;
    let path = Path::new(path_arg);

    let trash_path = Path::new(TRASH_PATH);
    let trash_info_path = Path::new(TRASH_INFO_PATH);


    match path.try_exists() {
        Ok(false) => {
            return Err(io::Error::new(io::ErrorKind::NotFound ,"The file or folder dosen't exist"));
        },
        Err(error) => {
            return Err(error);
        },
        _ => (),
    }

    let path = path::absolute(path)?;

    let name = path
        .file_name().ok_or(io::Error::other("Can't get the file or folder name"))?;


    let (file_trash_path, file_trash_info_path) = duplicate_name(name, trash_path, trash_info_path)?;

    let time = chrono::Local::now();


    std::fs::rename(&path, &file_trash_path)?;
    create_trash_info_file(&path, &file_trash_info_path, &time)?;


    // println!("{:?} -> {:?}", path, &target_path);
    Ok(())
}




fn create_trash_info_file(path: &Path, file_path: &Path, time: &DateTime<Local>) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let text = format!("[Trash Info]\nPath={}\nDeletionDate={}\n", path.to_string_lossy(), time.format("%Y-%m-%dT%H:%M:%S"));
    file.write_all(text.as_bytes())
}



// TODO: Horrible horribble code please FIXME FIXME please 🥺👉👈
fn duplicate_name(name: &OsStr, trash_path: &Path, trash_info_path: &Path) -> io::Result<(PathBuf, PathBuf)> {

    let file_trash_path = trash_path.join(name);
    let file_trash_path_str = file_trash_path.to_string_lossy().into_owned();

    let mut appendix = String::new();

    let mut n = 1;
    while Path::new(&(file_trash_path_str.clone() + &appendix)).try_exists()? {
        appendix = format!(" ({})", n);
        n += 1;
    }


    let file_trash_path_str = format!("{file_trash_path_str}{appendix}");
    let file_trash_path = Path::new(&file_trash_path_str).to_path_buf();

    let new_name = file_trash_path.file_name().ok_or(io::Error::other("Can't get new_filename from path"))?;
    let file_trash_info_path = trash_info_path.join(new_name);

    Ok((file_trash_path, file_trash_info_path))
}
