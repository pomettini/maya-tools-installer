extern crate reqwest;
extern crate os_type;
extern crate dirs;
extern crate serde;
extern crate serde_json;

use std::io::{Write};
use std::path::{PathBuf};
use std::fs::File;
use std::io;
use serde_json::{Error};

const AIV_SHELF_URL: &str = "https://www.giorgiopomettini.eu/aiv_shelf.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Shelf 
{
    pub response: String,
    pub shelf_url: String,
    pub shelf_name: String,
    pub icons_url: String,
    pub icons_name: Vec<String>,
    pub icons_extension: String,
    pub icons_variants: Vec<String>
}

#[derive(Debug, Default)]
pub struct Icon
{
    pub name: String,
    pub data: String
}

pub fn get_json_data() -> String
{
    match reqwest::get(AIV_SHELF_URL)
    {
        Ok(mut request) => 
        {
            match request.text()
            {
                Ok(text) => 
                {
                    write_log("Shelf data downloaded");
                    text
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
}

pub fn download_shelf_file(shelf: &Shelf) -> String
{
    match reqwest::get(&format!("{}{}", &shelf.shelf_url, &shelf.shelf_name))
    {
        Ok(mut request) => 
        {
            match request.text()
            {
                Ok(text) => 
                {
                    write_log("Shelf downloaded");
                    text
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
}

pub fn write_file(content: &str, path: &PathBuf) -> io::Result<()>
{
    let mut file = File::create(path)?;
    file.write_all(&content.as_bytes())?;
    Ok(())
}

pub fn get_shelf_data(data: &str) -> Result<Shelf, Error> 
{
    let shelf: Shelf = serde_json::from_str(data)?;
    Ok(shelf)
}

pub fn get_maya_directory() -> Option<PathBuf>
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

pub fn get_maya_shelf_directory(maya_path: &PathBuf, maya_version: &usize) -> Option<PathBuf>
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

pub fn get_maya_icons_directory(maya_path: &PathBuf, maya_version: &usize) -> Option<PathBuf>
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

pub fn get_maya_installed_versions(maya_path: &PathBuf) -> Vec<usize>
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

pub fn construct_icons_url(shelf: &Shelf) -> Vec<Icon>
{
    let mut icons: Vec<Icon> = Vec::new();

    for icon in &shelf.icons_name
    {
        for variant in &shelf.icons_variants
        {
            let mut i: Icon = Default::default();
            i.name = format!("{}{}.{}", &icon, &variant, &shelf.icons_extension);
            icons.push(i);
        }
    }

    icons
}

pub fn download_icons(shelf: &Shelf, icons: &mut Vec<Icon>)
{
    for icon in icons
    {
        write_log_new(&format!("Downloading icon {}", &icon.name));
        
        match reqwest::get(&format!("{}{}", &shelf.icons_url, &icon.name))
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
}

pub fn set_maya_directory() -> PathBuf
{
    match get_maya_directory()
    {
        Some(path) => 
        {
            write_log("Found Maya directory");
            path
        },
        None => 
        {
            write_log("Maya directory not found:");
            panic!();
        }
    }
}

pub fn check_json(json_data: Result<Shelf, Error>) -> Shelf
{
    match json_data
    {
        Ok(shelf_data) => 
        {
            if shelf_data.response == "OK"
            {
                write_log("Shelf data OK");
                shelf_data
            }
            else 
            {
                write_log("Shelf data error");
                panic!();
            }
        },
        Err(error) =>
        {
            write_log_new(&format!("Json cannot be parsed: {}", error));
            panic!();
        }
    }
}

// TODO Refactor with generics

pub fn write_log(content: &'static str)
{
    println!("{:?}", content);
}

pub fn write_log_new(content: &str)
{
    println!("{:?}", content);
}
