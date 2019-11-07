//系统常量
pub const START_DATE: &'static str = "2019-11-01";
pub const AUTHOR: &'static str = "Joshua Conero";
pub const VERSION: &'static str = "0.1.0";
pub const RELEASE: &'static str = "future";
pub const PROJECT: &'static str = "命令行解析程序";

pub mod cmd;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn args(){
        use std::env;
        println!("{:?}", env::args())
    }
}


