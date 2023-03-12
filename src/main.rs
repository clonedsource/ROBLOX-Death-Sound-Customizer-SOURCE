#![allow(non_snake_case)]

#[allow(unused_imports)]
use std::{env, fs, time::{Duration, SystemTime},};
use directories::{ProjectDirs};

#[allow(dead_code)]
const STRIPPED_PREFIX: &str = "\\\\?\\";

fn probe_path(path: &str) {
    
    match fs::metadata(path) {
        Ok(path_metadata) => {
            if path_metadata.is_dir() == false {
                match fs::create_dir(path) {
                    Ok(()) => {},
                    Err(_) => println!("Failed to create path."),
                }
            }
        },
        Err(_) => {
            match fs::create_dir(path) {
                Ok(()) => {},
                Err(_) => println!("Failed to create path."),
            }
        },
    }

}

fn main() {

    match env::current_dir() {
        Ok(exe_dir) => {
            let exe_dir_path: &str = exe_dir.as_path().to_str().unwrap();

            probe_path(exe_dir_path); // Fix later. And by "Fix", I mean concatenate the dir path for all the sounds used, and sound being directed.

            if let Some(rblx_appdata) = ProjectDirs::from("", "", "") {
                
                let mut rblx_appdata_path: String = String::new();

                let mut old_replaced_path: String = String::new();
                old_replaced_path.push_str(&exe_dir_path);
                old_replaced_path.push_str("\\old");
                probe_path(&old_replaced_path.as_str());

                let mut hit_replaced_path: String = String::new();
                hit_replaced_path.push_str(&exe_dir_path);
                hit_replaced_path.push_str("\\hit");
                probe_path(&hit_replaced_path.as_str());

                match rblx_appdata.data_local_dir().to_str().unwrap().strip_suffix(&"data") {
                    Some(stripped) => {
                        rblx_appdata_path.push_str(stripped);
                    },
                    _ => {println!("Failed")},
                } // Prevents it from panicking, also turns it into a String for my own benefit.

                rblx_appdata_path.push_str("Roblox\\Versions");

                let mut version_interval: u64 = 0_u64;
                for _version in fs::read_dir(&rblx_appdata_path).unwrap() {
                    match _version { // Is the version okay to mess with?
                        Ok(_version) => {
                            match fs::metadata(_version.path()) {
                                Ok(_version_metadata) => { // Is the metadata of the version valid?
                                    if (&_version_metadata).is_dir() { // Is it a directory?
                                        match (&_version_metadata).modified().unwrap().duration_since(SystemTime::UNIX_EPOCH) {
                                            Ok(_interval) => {

                                                if &_interval.as_secs() > &version_interval { // Basically finds the newest one.
                                                    version_interval = _interval.as_secs();
                                                    rblx_appdata_path = _version.path().to_str().unwrap().to_string();
                                                }
                                                
                                            },
                                            _ => {
                                                
                                            }
                                        }
                                    }
                                    
                                 
                                    
                                    
                                },
                                _ => {},
                            }
                            
                            
                        },
                        _ => {},
                    }
                    
                }
                rblx_appdata_path.push_str("\\content\\sounds\\ouch.ogg");
                //println!("{:?}", fs::read_dir(&rblx_appdata_path).unwrap().nth(0 as usize));

                for _file in fs::read_dir(&hit_replaced_path).unwrap() {
                    match _file { // Is the file okay to mess with?
                        Ok(_file) => {
                            match fs::metadata(_file.path()) {
                                Ok(hit_file) => { // Is the metadata of the file valid?
                                    if (&hit_file).is_file() { // Is it a file?
                                            let hit_file_path = _file.path().to_str().unwrap().to_string();
                                            let mut to_file_path: String = String::new();
                                            to_file_path += &old_replaced_path;
                                            to_file_path.push_str(&_file.path().to_str().unwrap().strip_prefix(&hit_replaced_path.as_str()).unwrap());
                                            
                                            match fs::copy(&hit_file_path.as_str(), &to_file_path.as_str()) {
                                                _ => {},
                                            }
                                            match fs::copy(&hit_file_path.as_str(), &rblx_appdata_path.as_str()) {
                                                _ => {},
                                            }

                                            break;
                                    }
                                },
                                _ => println!("Wow epic fail."),
                            }                            
                        },
                        _ => {},
                    }
                }
                

            }
            
            // Make a path link for ROBLOX Data Folder.
            // Copy new audio file to ROBLOX Data Folder.

        },
                                 
        Err(e) => println!("failed to get current exe path: {e}"),
    };
    
}
