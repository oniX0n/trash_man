use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::os::unix::fs::MetadataExt;

const REMOVE_DELTA_SCS: u64 = 5 * 24 * 60 * 60;
const TRASH_PATH: &str = "/home/theo/Trash/";
const DRY_RUN: bool = false;




fn main() {


    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Something horrible happened").as_secs();
    let trash_path = PathBuf::from(TRASH_PATH);

    remove_traverse(&trash_path , &now);
}


fn remove_traverse(item: &PathBuf, time_now: &u64) {


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
        if should_delete(&dir_entry, time_now, &REMOVE_DELTA_SCS) {
            remove(&dir_entry);
        } else {
            println!("Not Removing: {:?}, too young", dir_entry.path());
            remove_traverse(&dir_entry.path(), time_now);
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

fn remove(item: &DirEntry) {

    let file_type =  match item.file_type() {
        Ok(file_type) => file_type,
        Err(error) => {
            eprintln!("Can't get filetype for {:?}, Error {:?}", item.path(), error);
            return
        }
    };

    let remove_result = if file_type.is_dir() {
        if !DRY_RUN {
            println!("Remove: {:?}", item.path());
            fs::remove_dir_all(item.path())
        } else {
            println!("Dry Run: would remove: {:?}", item.path());
            Ok(())
        }
    } else if file_type.is_file() {
        if !DRY_RUN {
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
