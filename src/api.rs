use crate::config::Config;
use sha3::Digest;

pub struct Api {
    flag_l: i32,
    flag_p: bool,
    pub flag_s: bool,
    flag_u: bool,
    config: Config,
}

impl Api {
    pub fn new() -> Api {
        Api {
            flag_l: 25,
            flag_p: false,
            flag_s: false,
            flag_u: false,
            config: Config::new(),
        }
    }

    pub fn main(&mut self, mpass: &String, site: &String, id: &String, flags: &Vec<String>) {
        crate::config::Config::new();
        self.config = self.config.read_config_file();
        self.process_flags(flags);

        let mut hasher = sha3::Sha3_512::new();
        let mut seed: String;

        if self.flag_s {
            seed = mpass.to_string()
        } else {
            seed = mpass.to_string() + site + id;
        }
        if self.flag_u {
            seed += "USER_SALT";
        }

        hasher.update(seed);

        let mut hash = hasher.finalize();
        let mut pass = String::new();

        for i in &mut hash {
            let mut ch = ((*i as f32 * 93.0 / 254.0 + 33.0).round()) as u8;
            //print!("{}, ", ch);
            if self.flag_u
                && ((ch >= 97 && ch <= 122) || (ch >= 65 && ch <= 90) || (ch >= 48 && ch <= 57))
            {
                if ch >= 65 && ch <= 90 {
                    ch += 32;
                }
                pass += &char::from(ch).to_string();
            } else if !self.flag_u {
                pass += &char::from(ch).to_string();
            }
        }

        pass.truncate(self.flag_l as usize);
        self.output_pass(pass);
    }

    fn output_pass(&self, pass: String) {
        if self.flag_p {
            println!();
            println!("{}", pass);
        } else {
            std::process::Command::new(&self.config.process_cmd()[0])
                .args(&self.config.process_cmd()[1..])
                .arg(pass)
                .spawn()
                .expect("Failed to copy passwd to clipboard. NOTE: command \"wl-copy -o\" is used by default.");
        }
    }

    pub fn process_flags(&mut self, flags: &Vec<String>) {
        self.flag_l = self.config.out_len;
        for flag in flags {
            match flag.as_str() {
                "-h" | "--help" => {
                    Self::flag_h();
                }
                "-l" | "--len" => {
                    // get the current index of flag
                    if let Some(i) = flags.iter().position(|r| r == flag) {
                        if flags.len() > i {
                            self.flag_l = flags[i + 1].parse().unwrap();
                        }
                    }
                }
                "-p" | "--print" => self.flag_p = true,
                "-s" | "--single" => self.flag_s = true,
                "-u" | "--user" => self.flag_u = true,

                "-ps" => {
                    self.flag_p = true;
                    self.flag_s = true;
                }
                "-pu" => {
                    self.flag_p = true;
                    self.flag_u = true;
                }
                "-psu" => {
                    self.flag_p = true;
                    self.flag_s = true;
                    self.flag_u = true;
                }
                "-su" => {
                    self.flag_s = true;
                    self.flag_u = true;
                }
                _ => {}
            }
        }
    }

    fn flag_h() {
        println!(
            r"
The list for available flags is:

- `tapm -h`: print this menu.
- `tapm -l`: specify the desired lenght of the outupt. Overwrites the config property.
- `tapm -p`: print the output, prevents from copying to the clipbard.
- `tapm -s`: ask for a single entry.
- `tapm -u`: output a username.

For more info, check out the README.md
            "
        );
        std::process::exit(1);
    }
}
