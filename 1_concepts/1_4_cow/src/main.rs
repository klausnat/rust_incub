use std::borrow::Cow;
use std::env;

// TASK:
// Write a simple program which prints out the path to its configuration file. The path should be detected with the following precedence:

//     1.default path is /etc/app/app.conf;
//     2.if APP_CONF env var is specified (and not empty) then use it with higher priority than default;
//     3.if --conf command line argument is specified (error if empty) then use it with the highest priority.

// If neither APP_CONF env var nor --conf command line argument is specified, then no allocation should happen for path detection.

fn process(input: &str) -> Cow<str> {
    let args: Vec<String> = env::args().collect();
    let conf_pos_option = args.iter().position(|x| x == "conf");
    if conf_pos_option.is_some() {
        let conf_pos = conf_pos_option.unwrap();
        let arg_after_conf_option = args.get(conf_pos + 1);
        if arg_after_conf_option.is_some() {
            let res = arg_after_conf_option.unwrap();
            if !res.is_empty() {
                Cow::Owned(res.to_owned())
            } else {
                // conf value can not be empty. Just return default
                Cow::Borrowed(input)
            }
        } else {
            // conf value can not be empty. Just return default
            Cow::Borrowed(input)
        }
    } 
    else if env::var("APP_CONF").is_ok() {
        let res = env::var("APP_CONF").unwrap();
        if !res.is_empty() {
            Cow::Owned(res)
        } else {
            Cow::Borrowed(input)
        }

    }
    else {
        // If no APP_CONF env var or --conf command, just use default path
        Cow::Borrowed(input)
    }
}

fn main() {
    let default_path: &str = "/etc/app/app.conf";
    println!("path to configuration file: {}", process(default_path));
}
