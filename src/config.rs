use std::io::prelude::*;

#[derive(serde_derive::Deserialize)]
pub struct Config {
    pub copy_cmd: String,
    pub out_len: i32,
}

impl Config {
    pub fn new() -> Config {
        Config {
            copy_cmd: "".to_string(),
            out_len: 0,
        }
    }

    pub fn process_cmd(&self) -> Vec<&str> {
        self.copy_cmd.split(" ").collect()
    }

    pub fn read_config_file(&self) -> Config {
        if let Some(pro_dir) = directories::ProjectDirs::from("com", "github", "tapm") {
            let config_path = pro_dir.config_dir();

            // create config file if non existant
            if !std::fs::exists(config_path)
                .expect("ERROR: Failed to check if config directory exists.")
            {
                println!("No config file exists. Creating a new one.");
                let _ = std::fs::create_dir_all(config_path);
                let mut file =
                    std::fs::File::create(config_path.to_str().unwrap().to_owned() + "/tapm.toml")
                        .expect("ERROR: Failed to create config file.");
                let _ = file.write_all(
                    br#"copy_cmd = "wl-copy -o"
out_len = 25
"#,
                );
            }
            // read config
            let mut config_file =
                std::fs::File::open(config_path.to_str().unwrap().to_owned() + "/tapm.toml")
                    .expect("ERROR: Failed to open config file.");
            let mut config_string = String::new();
            config_file
                .read_to_string(&mut config_string)
                .expect("ERROR: Failed to read config file.");

            // parse config
            let config: Config =
                toml::from_str(&config_string).expect("ERROR: Failed to parse toml config.");
            config
        } else {
            std::process::exit(1);
        }
    }
}
