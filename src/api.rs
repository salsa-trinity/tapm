use sha3::Digest;

pub struct Api {
    flag_l: i32,
    flag_p: bool,
    flag_u: bool,
}

impl Api {
    pub fn new() -> Api {
        Api {
            flag_l: 25,
            flag_p: false,
            flag_u: false,
        }
    }

    pub fn main(&mut self, mpass: &String, site: &String, id: &String, flags: &Vec<String>) {
        self.process_flags(flags);

        let mut hasher = sha3::Sha3_512::new();
        if !self.flag_u {
            hasher.update(mpass.to_owned() + site + id);
        } else {
            hasher.update(mpass.to_owned() + site + id + "USER_SALT");
        }
        let mut hash = hasher.finalize();
        let mut pass = String::new();

        //println!("{}", mpass);
        //println!("{}", site);
        //println!("{}", id);
        //println!("{:?}", flags);
        //println!("{:?}", self.flag_u);

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
        //println!();
        //println!("hash: {}", pass);
        pass.truncate(self.flag_l as usize);
        self.outup_pass(pass);
    }

    fn outup_pass(&self, pass: String) {
        if self.flag_p {
            println!();
            println!("{}", pass);
        } else {
            std::process::Command::new("wl-copy")
                .arg("-o")
                .arg(pass)
                .spawn()
                .expect("Failed to copy passwd to clipboard");
        }
    }

    pub fn process_flags(&mut self, flags: &Vec<String>) {
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
                "-u" | "--user" => self.flag_u = true,
                "-pu" => {
                    self.flag_p = true;
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
- `tapm -p`: print the output, prevents from copying to the input.
- `tapm -u`: output a username.

For more info, check out the README.md
            "
        );
        std::process::exit(1);
    }
}
