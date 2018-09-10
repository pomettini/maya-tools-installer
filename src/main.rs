extern crate reqwest;
extern crate os_type;
extern crate dirs;

use std::io::{Write};
use std::path::{ PathBuf};

use std::fs::File;
use std::io;

const SHELF_URL: &'static str = "https://cdn.rawgit.com/Pomettini/maya-tools/8b6519c3/shelf/shelf_AIV.mel";
const SHELF_FILE_NAME: &'static str = "shelf_AIV.mel";

fn main() 
{
    let shelf_content: String;

    // Check if remote shelf file exists

    // Download shelf file
    match reqwest::get(SHELF_URL)
    {
        Ok(mut request) => 
        {
            match request.text()
            {
                Ok(text) => 
                {
                    write_log("Shelf downloaded");
                    shelf_content = text;
                },
                Err(error) => 
                {
                    write_log_new(format!("Shelf downloaded but got error: {}", error));
                    panic!();
                }
            }
        },
        Err(error) => 
        {
            write_log_new(format!("Error downloading shelf: {}", error));
            panic!();
        }
    }

    // Check CRC (optional)

    let mut maya_directory = PathBuf::new();

    // Get Maya directory
    // Check if Maya directory exists
    match get_maya_directory()
    {
        Some(path) => 
        {
            write_log("Found Maya directory");
            maya_directory = path;
        },
        None => 
        {
            write_log("Maya directory not found");
            panic!();
        }
    }

    // Check which versions of Maya are installed
    let maya_installed_versions = get_maya_installed_versions(&maya_directory);
    // For each Maya version:
    for maya_version in maya_installed_versions
    {
        write_log_new(format!("Now working on Maya version {}", maya_version));

        let mut maya_shelf_directory = PathBuf::new();

        // Get Maya shelf directory
        // Check if Maya shelf directory exists
        match get_maya_shelf_directory(&maya_directory, &maya_version)
        {
            Some(path) => 
            {
                write_log_new(format!("Found shelf directory for Maya {}, moving on", maya_version));
                maya_shelf_directory = path;
            },
            None => 
            {
                write_log_new(format!("There is no shelf directory for Maya {}, moving to next version", maya_version));
                continue;
            }
        }

        // Get complete shelf path with filename and extension
        let mut maya_file_shelf_path = PathBuf::from(&maya_shelf_directory);
        maya_file_shelf_path.push(SHELF_FILE_NAME);

        // Write shelf file
        match write_file(&shelf_content, &maya_file_shelf_path)
        {
            Ok(result) => 
            {
                write_log("Write complete!");
            },
            Err(error) => 
            {
                write_log_new(format!("Could not write on the directory: {}", error));
            }
        }

        // Check if shelf file exist
    }
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

    if maya_directory.exists()
    {
        Some(maya_directory)
    }
    else 
    {
        None
    }
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
            // Inefficent, needs refactor
            for version in 2011..2020 
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

fn write_file(content: &String, path: &PathBuf) -> io::Result<()>
{
    let mut file = File::create(path)?;
    file.write_all(&content.as_bytes())?;
    Ok(())
}

// TODO Refactor with generics

fn write_log(content: &'static str)
{
    println!("{:?}", content);
}

fn write_log_new(content: String)
{
    println!("{:?}", content);
}