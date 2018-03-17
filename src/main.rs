use std::env;
use std::borrow::Cow;

fn main() {
    let pwd = env::var_os("PWD")
        .expect("$PWD is not set in the environment!")
        .into_string()
        .expect("$PWD does not contain a valid string!");
    let home = env::var_os("HOME")
        .expect("$HOME is not set in the environment!")
        .into_string()
        .expect("$HOME does not contain a valid string!");
    println!("{}", remove_home(&home, &pwd));
}

fn remove_home<'a>(home: &str, pwd: &'a str) -> Cow<'a, str>{
    if pwd.starts_with(home) {
        pwd.replace(home, "~").into()
    } else {
        pwd.into()
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
}
