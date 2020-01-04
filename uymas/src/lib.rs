//系统常量
pub const START_DATE: &'static str = "2019-11-01";
pub const AUTHOR: &'static str = "Joshua Conero";
pub const VERSION: &'static str = "0.2.0";
pub const RELEASE: &'static str = "2020-01-04";
pub const PROJECT: &'static str = "uymas";

pub mod cmd;

#[cfg(test)]
mod tests {
    use crate::cmd;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn args() {
        use std::env;
        println!("{:?}", env::args())
    }

    #[test]
    fn test_cmd_split_str_kv() {
        // case1
        let (idx, kv) = cmd::split_str_kv(String::from("key=value"), '=');
        println!("{} => {:?}", idx, kv);

        // case2
        let (idx, kv) = cmd::split_str_kv(String::from("name='Joshua Conero'"), '=');
        println!("{} => {:?}", idx, kv);
    }

    #[test]
    fn test_cmd_str_trim_quote() {
        let s = cmd::str_trim_quote(String::from("'test in the centos!'"));
        println!("{}", s);

        let s = cmd::str_trim_quote(String::from("\"test in the centos!\""));
        println!("{}", s);
    }
}
