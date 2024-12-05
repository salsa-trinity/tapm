struct Config {
    copy_cmd: String,
    pass_len: i32,
    user_len: i32,
}

impl Config {
    pub fn new() -> Config {
        let copy_cmd = "wl-copy -o".to_string();
        let pass_len = 25;
        let user_len = 25;
        Config {
            copy_cmd,
            pass_len,
            user_len,
        }
    }
}
