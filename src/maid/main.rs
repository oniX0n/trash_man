use std::path::Path;
use std::env;


const TRASH_PATH: &str = "/home/theo/test";


fn main()  {

    let args: Vec<String> = env::args().collect();

    let path_string = &args[1];
    let path = Path::new(path_string);

    match path.try_exists() {
        Ok(false) => {
            eprintln!("The File or Folder doesn't exist");
            return;
        },
        Err(error) => {
            eprintln!("Can't access path: {:?}", error);
            return
        },
        _ => (),
    }

    let absolute_path = std::path::absolute(path).expect("Path is invalid");


    let absolute_path_str = absolute_path.to_str().expect("Not unicode");
    let target_string = format!("{}{}", TRASH_PATH, absolute_path_str);
    let target_path = Path::new(&target_string);

    std::fs::create_dir_all(target_path.parent().unwrap()).expect("Coudnt");
    std::fs::rename(path, target_path).expect("Yo dawg aint working");
    println!("{:?} -> {:?}", path, &target_path);
}
