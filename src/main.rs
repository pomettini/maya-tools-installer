extern crate reqwest;
extern crate os_type;
extern crate dirs;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::io::{Write};
use std::path::{PathBuf};
use std::fs::File;
use std::io;
use serde_json::{Error};

const AIV_SHELF_URL: &str = "https://www.giorgiopomettini.eu/aiv_shelf.json";

#[derive(Serialize, Deserialize, Debug)]
struct Shelf 
{
    Response: String,
    ShelfURL: String,
    ShelfName: String,
    IconsURL: String,
    IconsName: Vec<String>,
    IconsExtension: String,
    IconsVariants: Vec<String>
}

#[derive(Debug, Default)]
struct Icon
{
    name: String,
    data: String
}

fn main() 
{
    let shelf: Shelf;
    let mut icons: Vec<Icon> = Vec::new();
    let shelf_data: String;

    // Get Json data
    match reqwest::get(AIV_SHELF_URL)
    {
        Ok(mut request) => 
        {
            match request.text()
            {
                Ok(text) => 
                {
                    write_log("Shelf data downloaded");
                    shelf_data = text;
                },
                Err(error) => 
                {
                    write_log_new(&format!("Shelf data downloaded but got error: {}", error));
                    panic!();
                }
            }
        },
        Err(error) => 
        {
            write_log_new(&format!("Error downloading shelf data: {}", error));
            panic!();
        }
    }

    // Parse Json data
    let json_data = get_shelf_data(&shelf_data);

    // Check if Json data is ok
    match json_data
    {
        Ok(shelf_data) => 
        {
            if shelf_data.Response == "OK"
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

    let shelf_content: String;

    // Check if remote shelf file exists

    // Download shelf file
    match reqwest::get(&format!("{}{}", &shelf.ShelfURL, &shelf.ShelfName))
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
                    write_log_new(&format!("Shelf downloaded but got error: {}", error));
                    panic!();
                }
            }
        },
        Err(error) => 
        {
            write_log_new(&format!("Error downloading shelf: {}", error));
            panic!();
        }
    }

    // Check shelf file CRC (optional)

    // Constructing Icons urls
    for icon in &shelf.IconsName
    {
        for variant in &shelf.IconsVariants
        {
            let mut i: Icon = Default::default();
            i.name = format!("{}{}.{}", &icon, &variant, &shelf.IconsExtension);
            icons.push(i);
        }
    }

    // Download icons
    for icon in &mut icons
    {
        write_log_new(&format!("Downloading icon {}", &icon.name));

        match reqwest::get(&format!("{}{}", &shelf.IconsURL, &icon.name))
        {
            Ok(mut request) => 
            {
                match request.text()
                {
                    Ok(data) => 
                    {
                        write_log_new(&format!("Icon {} downloaded", &icon.name));
                        icon.data = data;
                    },
                    Err(error) => 
                    {
                        write_log_new(&format!("Icon downloaded but got error: {}", error));
                    }
                }
            },
            Err(error) => 
            {
                write_log_new(&format!("Error downloading icon: {}", error));
            }
        }
    }

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
        maya_file_shelf_path.push(&shelf.ShelfName);

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
                write_log_new(&format!("Could not write on the directory: {}", error));
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
    }

    // Close and do stuff
    write_log("Installation complete");
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
        os_type::OSType::OSX => 
        {
            maya_directory.push("Library");
            maya_directory.push("Preferences");
            maya_directory.push("Autodesk");
            maya_directory.push("maya");
        },
        // This will probably be Windows, or maybe not
        _ => 
        {
            maya_directory.push("Documents");
            maya_directory.push("maya");
        }
    }

    println!("Maya directory: {:?}", &maya_directory);

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

fn get_maya_icons_directory(maya_path: &PathBuf, maya_version: &usize) -> Option<PathBuf>
{
    let mut icons_directory = PathBuf::new();

    icons_directory.push(&maya_path);
    icons_directory.push(maya_version.to_string());
    icons_directory.push("prefs");
    icons_directory.push("icons");

    if icons_directory.exists()
    {
        Some(icons_directory)
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

fn write_file(content: &str, path: &PathBuf) -> io::Result<()>
{
    let mut file = File::create(path)?;
    file.write_all(&content.as_bytes())?;
    Ok(())
}

fn get_shelf_data(data: &str) -> Result<Shelf, Error> 
{
    let shelf: Shelf = serde_json::from_str(data)?;
    Ok(shelf)
}

// TODO Refactor with generics

fn write_log(content: &'static str)
{
    println!("{:?}", content);
}

fn write_log_new(content: &str)
{
    println!("{:?}", content);
}
