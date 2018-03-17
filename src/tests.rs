#[test]
fn remove_home() {
    let home = "/home/kyle"
    let pwd_with_home = "/home/kyle/Documents"
    let pwd_without_home = "/home/potato/Documents"
    assert_equal!(remove_home(home, pwd_with_home), "~/Documents")
    assert_equal!(remove_home(home, pwd_without_home), pwd_without_home)
}
