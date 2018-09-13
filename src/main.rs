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
    // Get Json data
    let json = get_json_data();

    // Parse Json data
    let parsed_json = get_shelf_data(&json);

    // Check if Json data is ok
    let shelf = check_json(parsed_json);

    // Download shelf file
    let shelf_content = download_shelf_file(&shelf);

    // Check shelf file CRC (optional)

    // Constructing Icons urls
    let mut icons = construct_icons_url(&shelf);

    // Download icons
    download_icons(&shelf, &mut icons);

    // Get Maya directory
    // Check if Maya directory exists
    let maya_directory = set_maya_directory();

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
            write_log("Shelf already exists, will be overwritten");
        }

        // Write shelf file
        match write_file(&shelf_content, &maya_file_shelf_path)
        {
            Ok(()) => 
            {
                write_log("Shelf writing complete");
            },
            Err(error) => 
            {
                write_log_new(&format!("Could not write shelf on the directory {:?}: {}", &maya_file_shelf_path, error));
            }
        }

        let mut maya_icons_directory = PathBuf::new();

        // Get Maya icons directory
        // Check if Maya icons directory exists
        match get_maya_icons_directory(&maya_directory, &maya_version)
        {
            Some(path) => 
            {
                write_log_new(&format!("Found icons directory for Maya {}, moving on", maya_version));
                maya_icons_directory = path;
            },
            None => 
            {
                write_log_new(&format!("There is no icons directory for Maya {}, moving to the next version", maya_version));
                continue;
            }
        }

        // For each icon
        for icon in &icons
        {
            // Get complete icon path with filename and extension
            let mut icon_path = PathBuf::from(&maya_icons_directory);
            icon_path.push(&icon.name);
            
            // Check if icon file exists
            if icon_path.exists()
            {
                write_log_new(&format!("File at {:?} already exists, will be overwritten", &icon_path));
            }

            // Write icon file
            match write_file_binary(&icon.data, &icon_path)
            {
                Ok(()) => 
                {
                    write_log_new(&format!("Writing icon {} complete", &icon.name));
                },
                Err(error) => 
                {
                    write_log_new(&format!("Could not write icon on the directory {:?}: {}", &icon_path, error));
                }
            }
        }
    }

    // Close and do stuff
    write_log("Installation complete");
}
