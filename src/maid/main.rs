use std::path::Path;
use std::env;
use std::io;

const TRASH_PATH: &str = "/home/theo/Trash";


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: maid FILE...");
    }

    for i in 1..args.len() {
        let arg = &args[i];
        if let Err(error) = run(arg) { eprintln!("Error: {}", error) }
    }

}

fn run(arg: &String) -> io::Result<()> {

    let path_arg = arg;
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

    let target_str = format!("{}/{}-{}", TRASH_PATH, time.format("%d-%m-%H:%M:%S"), name);
    let target_path = Path::new(&target_str);

    std::fs::create_dir_all(target_path.parent().ok_or(io::Error::other("Couldn't get parent of path"))?)?;
    std::fs::rename(path, target_path)?;

    println!("{:?} -> {:?}", path, &target_path);
    Ok(())
}
