
pub struct Cmd{
    args: Vec<str>
}

// 命令行
impl Cmd{
    // 构造函数
    pub fn new() -> Cmd{
        Cmd{ args: vec![] }
    }

    // 运行
    pub fn run(self){}
}


