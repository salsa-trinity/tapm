pub struct Config {
    pub copy_cmd: String,
    pub out_len: i32,
}

impl Config {
    pub fn new() -> Config {
        // ********** CONFIG **********
        let copy_cmd = "wl-copy -o".to_string();
        let out_len = 25;

        Config { copy_cmd, out_len }
    }
    //pub fn process_cmd(&self) -> Vec<String> {}
}
