use std::env::args;
use std::fs::{create_dir, rename};

fn main() {
    let args: Vec<String> = args().collect();
    let rm_file_path = &args[1];

    let array_path_to_file: Vec<_> = rm_file_path.split('/').collect();

    let array_path_to_folder = &array_path_to_file[..array_path_to_file.len() - 1];

    let mut array_new_file_path = array_path_to_file.to_vec();
    let mut array_trash_file_path = array_path_to_folder.to_vec();

    if array_path_to_file.len() > 1 {
        array_new_file_path.insert(array_path_to_file.len() - 1, "trash");
        array_trash_file_path.insert(array_trash_file_path.len(), "trash");
    }

    let _ = create_dir(array_trash_file_path.join("/"));

    match rename(array_path_to_file.join("/"), array_new_file_path.join("/")) {
        Err(why) => println!("Error: {}", why.kind()),
        Ok(_) => {},
    }
}
