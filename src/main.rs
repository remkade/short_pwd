#[macro_use]
extern crate structopt;

use std::env;
use std::borrow::Cow;
use structopt::StructOpt;

/// A simple pwd shortener.
#[derive(StructOpt, Debug)]
#[structopt(name = "short_pwd")]
struct Opt {
    /// The separator to replace the usual '/' with
    #[structopt(short = "s", long = "separator", default_value="/")]
    separator: String,
}

fn main() {
    // Get all the necessary env vars
    let pwd = env::var_os("PWD")
        .expect("$PWD is not set in the environment!")
        .into_string()
        .expect("$PWD does not contain a valid string!");
    let home = env::var_os("HOME")
        .expect("$HOME is not set in the environment!")
        .into_string()
        .expect("$HOME does not contain a valid string!");

    // Parse commandline arguments
    let opt = Opt::from_args();

    let new_pwd = remove_home(&home, &pwd);

    println!("{}", replace_separators(&opt.separator, &new_pwd));
}

fn remove_home<'a>(home: &str, pwd: &'a str) -> Cow<'a, str>{
    if pwd.starts_with(home) {
        pwd.replace(home, "~").into()
    } else {
        pwd.into()
    }
}

fn replace_separators<'a>(separator: &str, pwd: &'a str) -> Cow<'a, str>{
    if separator == "/" {
        pwd.into()
    } else {
        pwd.replace("/", separator).into()
    }
}

#[cfg(test)]
mod tests {
    use remove_home;
    #[test]
    fn remove_home_test() {
        let home = "/home/kyle";
        let pwd_with_home = "/home/kyle/Documents";
        let pwd_without_home = "/home/potato/Documents";
        assert_eq!(remove_home(home, pwd_with_home), "~/Documents");
        assert_eq!(remove_home(home, pwd_without_home), pwd_without_home);
    }

    use replace_separators;
    #[test]
    fn replace_separators_test() {
        let pwd = "/home/kyle";
        assert_eq!(replace_separators("!", pwd), "!home!kyle");
        assert_eq!(replace_separators("🤠", pwd), "🤠home🤠kyle");
    }
}
