extern crate reqwest;
extern crate os_type;
extern crate dirs;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::path::{PathBuf};

pub mod installer;

use installer::*;

fn main() 
{
    let shelf: Shelf;
    let mut shelf_data = String::new();

    // Get Json data
    shelf_data = get_json_data();

    // Parse Json data
    let json_data = get_shelf_data(&shelf_data);

    // Check if Json data is ok
    match json_data
    {
        Ok(shelf_data) => 
        {
            if shelf_data.response == "OK"
            {
                write_log("Shelf data OK");
                shelf = shelf_data;
            }
            else 
            {
                write_log("Shelf data error");
                panic!();
            }
        },
        Err(error) =>
        {
            write_log("Json cannot be parsed");
            panic!();
        }
    }

    // Download shelf file
    let shelf_content = download_shelf_file(&shelf);

    // Check shelf file CRC (optional)

    // Constructing Icons urls
    let mut icons = construct_icons_url(&shelf);

    // Download icons
    download_icons(&shelf, &mut icons);

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
            write_log("Maya directory not found:");
            panic!();
        }
    }

    // Check which versions of Maya are installed
    let maya_installed_versions = get_maya_installed_versions(&maya_directory);
    // For each Maya version:
    for maya_version in maya_installed_versions
    {
        write_log_new(&format!("Now working on Maya version {}", maya_version));

        let mut maya_shelf_directory = PathBuf::new();

        // Get Maya shelf directory
        // Check if Maya shelf directory exists
        match get_maya_shelf_directory(&maya_directory, &maya_version)
        {
            Some(path) => 
            {
                write_log_new(&format!("Found shelf directory for Maya {}, moving on", maya_version));
                maya_shelf_directory = path;
            },
            None => 
            {
                write_log_new(&format!("There is no shelf directory for Maya {}, moving to the next version", maya_version));
                continue;
            }
        }

        // Get complete shelf path with filename and extension
        let mut maya_file_shelf_path = PathBuf::from(&maya_shelf_directory);
        maya_file_shelf_path.push(&shelf.shelf_name);

        // Check if shelf file exist
        if maya_file_shelf_path.exists()
        {
            write_log("File already exists, will be overwritten");
        }

        // Write shelf file
        match write_file(&shelf_content, &maya_file_shelf_path)
        {
            Ok(()) => 
            {
                write_log("Write complete");
            },
            Err(error) => 
            {
                write_log_new(&format!("Could not write on the directory {:?}: {}", &maya_file_shelf_path, error));
            }
        }

        // Check if shelf file has been written
        if maya_file_shelf_path.exists()
        {
            write_log("File has been written, moving on");
        }
        else 
        {
            write_log("File has not been written");
        }

        // let mut maya_icons_directory = PathBuf::new();

        // // Get Maya icons directory
        // match get_maya_icons_directory(&maya_directory, &maya_version)
        // {
        //     Some(path) => 
        //     {
        //         write_log_new(&format!("Found icons directory for Maya {}, moving on", maya_version));
        //         maya_icons_directory = path;
        //     },
        //     None => 
        //     {
        //         write_log_new(&format!("There is no icons directory for Maya {}, moving to the next version", maya_version));
        //         continue;
        //     }
        // }

        // let mut maya_icons_path = PathBuf::from(&maya_directory);
        

        // Check if Maya icons directory exists

        // For each icon
        // Get complete icon path with filename and extension

        // Check if icon file ecits

        // Write icon file

        // Check if shelf file has been written
    }

    // Close and do stuff
    write_log("Installation complete");
}
