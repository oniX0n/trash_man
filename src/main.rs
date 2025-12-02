use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::os::unix::fs::MetadataExt;
use std::env;

const TRASH_PATH: &str = "/home/theo/test/";
const DELETE_DELTA: u64 = 30;


struct Options {
    verbose: bool,
    dry_run: bool,
}
impl Default for Options {
    fn default() -> Self {
       Options { verbose: true, dry_run: true }
    }
}

fn main() {
    let mut options = Options::default();

    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("--nv")) {
        options.verbose = false;
    }
    if args.contains(&String::from("--dry-run")) {
        options.dry_run = true;
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Something horrible happened").as_secs();
    let path = PathBuf::from(TRASH_PATH);

    remove_traverse(&path , &now, &DELETE_DELTA, &options);
}


fn remove_traverse(item: &PathBuf, time_now: &u64, treshold_secs: &u64, options: &Options) {


    if !item.is_dir() { return }


    let read_dir_iterator = match fs::read_dir(item) {
        Ok(iterator) => iterator,
        Err(error) => {
            eprintln!("Can't read directory: {:?}, Error: {:?}", item, error);
            return
        },
    };

    for dir_entry in read_dir_iterator {
        let dir_entry = match dir_entry {
            Ok(dir_entry) => dir_entry,
            Err(error) => {
                eprintln!("Can't get directory entry in {:?}, Error: {:?}", item, error);
                continue
            }
        };
        if should_delete(&dir_entry, time_now, treshold_secs) {
            remove(&dir_entry, options);
        } else {
            println!("Not Removing: {:?}, too young", dir_entry.path());
            remove_traverse(&dir_entry.path(), time_now, treshold_secs, options);
        }
    }
}


fn should_delete(item: &DirEntry, time_now: &u64, treshold_secs: &u64) -> bool {
    match item.metadata() {
        Ok(metadata) => {
            let ctime = metadata.ctime() as u64;
            (*time_now - ctime) > *treshold_secs
        },
        Err(error) => {
            eprintln!("Can't get metadata for: {:?}, Error: {:?}", item.path(), error);
            false
        },
    }
}

fn remove(item: &DirEntry, options: &Options) {

    let file_type =  match item.file_type() {
        Ok(file_type) => file_type,
        Err(error) => {
            eprintln!("Can't get filetype for {:?}, Error {:?}", item.path(), error);
            return
        }
    };

    let remove_result = if file_type.is_dir() {
        if !options.dry_run {
            println!("Remove: {:?}", item.path());
            fs::remove_dir_all(item.path())
        } else {
            println!("Dry Run: would remove: {:?}", item.path());
            Ok(())
        }
    } else if file_type.is_file() {
        if !options.dry_run {
            println!("Remove: {:?}", item.path());
            fs::remove_file(item.path())
        } else {
            println!("Dry Run: would remove: {:?}", item.path());
            Ok(())
        }
    } else {
        Err(std::io::Error::other("Neither Directory nor File"))
    };

    if let Err(error) = remove_result {
        eprintln!("Can't remove: {:?}, Error {:?}", item.path(), error);
    }
}
