extern crate curl;
extern crate os_type;
extern crate dirs;

use std::io::{stdout, Write};
use std::path::{PathBuf};

use curl::easy::Easy;

fn main() 
{
    let mut maya_valid_install_directories: Vec<PathBuf> = Vec::new();

    let maya_dir = get_maya_directory().unwrap();
    let maya_versions = get_maya_installed_versions(&maya_dir);

    for version in maya_versions
    {
        let mut maya_complete_path = PathBuf::new();

        maya_complete_path.push(&maya_dir);

        let s = get_maya_shelf_directory(&maya_dir, &version);

        match s
        {
            Some(s) => 
            {
                maya_complete_path.push(s);
                println!("Adding Maya {} to the valid install directories", &version);
                maya_valid_install_directories.push(maya_complete_path);
            },
            None => println!("Shelf directory doesn't exist for version {}", &version)
        }
    }

    // let mut easy = Easy::new();
    // easy.url("https://cdn.rawgit.com/Pomettini/maya-tools/8b6519c3/shelf/shelf_AIV.mel").unwrap();
    // easy.write_function(|data| {
    //     stdout().write_all(data).unwrap();
    //     Ok(data.len())
    // }).unwrap();
    // easy.perform().unwrap();

    // println!("{}", easy.response_code().unwrap());
}

fn get_maya_directory() -> Option<PathBuf>
{
    let mut maya_directory = PathBuf::new();

    match dirs::home_dir()
    {
        Some(path) => maya_directory.push(path),
        None => panic!("Cannot get your HOME dir"),
    }

    match os_type::current_platform().os_type 
    {
        os_type::OSType::OSX => maya_directory.push("Library/Preferences/Autodesk/maya"),
        // This will probably be Windows, or maybe not
        _ => maya_directory.push("\\Documents\\maya\\")
    }

    Some(maya_directory)
}

fn get_maya_shelf_directory(maya_path: &PathBuf, maya_version: &usize) -> Option<PathBuf>
{
    let mut shelf_directory = PathBuf::new();

    shelf_directory.push(&maya_path);
    shelf_directory.push(maya_version.to_string());
    shelf_directory.push("prefs");
    shelf_directory.push("shelves");

    if shelf_directory.exists()
    {
        Some(shelf_directory)
    }
    else 
    {
        None
    }
}

fn get_maya_installed_versions(maya_path: &PathBuf) -> Vec<usize>
{
    let mut maya_versions = Vec::new();

    for entry in maya_path.read_dir().unwrap()
    {
        if let Ok(entry) = entry 
        {
            for version in 2011..2030 
            {
                if entry.path().ends_with(version.to_string())
                {
                    maya_versions.push(version);
                }
            }
        }
    }

    maya_versions
}
