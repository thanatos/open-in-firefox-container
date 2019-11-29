#[cfg(target_os = "macos")]
#[link(name = "Foundation", kind = "framework")]
extern {}
#[cfg(target_os = "macos")]
#[link(name = "CoreServices", kind = "framework")]
extern {}
#[cfg(target_os = "macos")]
#[link(name = "AppKit", kind = "framework")]
extern {}

use std::fs::File;
use std::io::Read;
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize)]
struct ContainerUrl<'a> {
    name: &'a str,
    url: &'a str,
}


fn open_url_in_container(firefox: &str, url: &str, container: &str) {
    let encoded = serde_urlencoded::to_string(ContainerUrl {
        name: container,
        url: url,
    }).unwrap();
    let containered_url = format!("ext+container:{}", encoded);
    Command::new(firefox)
        .arg(&containered_url)
        .stdin(Stdio::null())
        .spawn()
        .expect("Failed to start firefox.")
    ;
}


#[derive(Debug, Deserialize)]
struct Config {
    firefox_command: String,
    container: String,
}

impl Config {
    fn load() -> Config {
        let config_path = {
            let mut path = dirs::config_dir()
                .expect("Couldn\'t determine config dir")
            ;
            path.push("open-in-firefox-container.toml");
            path
        };
        let mut config = String::new();
        File::open(&config_path)
            .expect("No config file found.")
            .read_to_string(&mut config)
            .unwrap()
        ;
        toml::from_str(&config).expect("Failed to read config file")
    }
}


#[cfg(not(target_os = "macos"))]
mod unix_main {
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    #[structopt(
        name = "open-in-firefox-container",
        about = "Open a URL in a container in Firefox"
    )]
    struct Args {
        url: String,
    }

    pub fn main() {
        let args = Args::from_args();
        let config = super::Config::load();
        super::open_url_in_container(
            &config.firefox_command,
            &args.url,
            &config.container,
        );
    }
}
#[cfg(target_os = "macos")]
mod osx_main;

#[cfg(not(target_os = "macos"))]
fn main() {
    unix_main::main()
}
#[cfg(target_os = "macos")]
fn main() {
    osx_main::main()
}
