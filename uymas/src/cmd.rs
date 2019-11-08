use std::env;

pub struct Cmd{
//    args: Vec<str>
}

// 命令行
impl Cmd{
    // 构造函数
    pub fn new() -> Cmd{
//        Cmd{ args: Vec::new() }
        Cmd{}
    }

    // 运行
    pub fn run(&self){
        // args 为生成器
        println!("{:?}", env::args());

        for arg in env::args(){
            println!("{}", arg)
        }
    }
}


