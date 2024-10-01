#![windows_subsystem = "console"]
#[cfg(target_feature = "crt-static")]
extern crate msvcrt;

use std::io;
use std::process::{Command};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
struct KobutskyConfig {
    path: String,
    arguments: Vec<String>,
}

const APP_NAME: &'static str = "kobutsky_runner";
const DEFAULT_CONFIG_SECTION: &'static str = "run";
const DOTNET_CLICK_ONCE_APP: &'static str = "X:\\ForStudents\\ForStudentWpf.application";

fn main() -> Result<(), io::Error> {
    let cfg_to_run: KobutskyConfig = confy::load(APP_NAME, DEFAULT_CONFIG_SECTION)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    if cfg_to_run.path.is_empty() {
        config_not_found()
    } else {
        // if !cfg!(dev) {
        //     Ok(run_with_stdout(cfg_to_run))
        // } else {
        //     //dbg!(&cfg);
        run_child(cfg_to_run)
        // }
    }
}

fn run_child(cfg: KobutskyConfig) -> Result<(), io::Error> {
    //dbg!(&cfg);
    let path = cfg.path.clone();
    let mut child = Command::new(cfg.path)
        .args(cfg.arguments)
        .spawn()
        .expect(&format!("Command failed to start! \
            Command: {}", path));

    let status = child.wait()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command failed to execute! \n \
                Command: `{}`", path)));
    } else {
        println!("Command `{}` executed successfully!", path);
    }

    return Ok(());
}

// fn run_child_with_raw_args(cfg: KobutskyConfig) -> Result<(), io::Error> {
//     //dbg!(&cfg);
//     let path = cfg.path.clone();
//     let mut child = Command::new(cfg.path)
//         .args(cfg.arguments)
//         .spawn()
//         .expect(&format!("Command failed to start! \
//             Command: {}", path));
//
//     let status = child.wait()?;
//
//     if !status.success() {
//         return Err(io::Error::new(
//             io::ErrorKind::Other,
//             format!("Command failed to execute! \n \
//                 Command: `{}`", path)));
//     } else {
//         println!("Command `{}` executed successfully!", path);
//     }
//
//     return Ok(());
// }
//
// fn run_with_stdout(cfg: KobutskyConfig) {
//     let path = cfg.path.clone();
//     let child = Command::new(cfg.path)
//         .args(cfg.arguments)
//         .output()
//         .expect(&format!("Command failed to start! \
//             Command: {}", path));
//
//     //let status = child.wait()?;
//     let output = child.stderr;
//
//     println!("Output: {:?}", output);
// }


fn config_not_found() -> Result<(), io::Error> {
    eprintln!("Config file was not found! \n Trying to create a new config file!");
    // Create and save a new config file
    // let default_app = "c:\\WINDOWS\\system32\\calc.exe".to_string();
    let default_app_args = vec![
        "-launchApplication".to_string(),
        DOTNET_CLICK_ONCE_APP.to_string()];
    let default_app = "C:\\WINDOWS\\System32\\PresentationHost.exe".to_string();
    let new_cfg = KobutskyConfig {
        path: default_app,
        arguments: default_app_args,
    };
    confy::store(APP_NAME, DEFAULT_CONFIG_SECTION, &new_cfg)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    if new_cfg.path.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound,
                                  "Could not create default config file! \n \
                                      FATAL ERROR!"));
    }

    // Load the config back
    let loaded_cfg: KobutskyConfig = confy::load(APP_NAME, DEFAULT_CONFIG_SECTION)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    if loaded_cfg.path.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound,
                                  "Failed to load newly created default config file!\n \
                                      FATAL ERROR!"));
    }

    println!("New config file was created!\n \
                                               Please update the path in the config file, \
                                               located in the following path: \
                                               ~/AppData/Roaming/{}/config/{}.toml\n",
             APP_NAME,
             DEFAULT_CONFIG_SECTION);
    return Ok(());
}
