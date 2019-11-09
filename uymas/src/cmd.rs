use std::env;

pub struct Cmd{
//    args: Vec<str>
    pub command: String,            // 命令
    _args: Vec<String>              // 内部参数，默认由系统参数读取
}

// 命令行
impl Cmd{
    // 构造函数
    pub fn new() -> Cmd{
        Cmd{
            command: String::from(""),
            _args: Vec::new()
        }
    }

    // 内部参数初始化，使用参数默认配置信息
    fn _init_args(&mut self) -> &mut Cmd {
        if self._args.len() < 1 {
            let mut args: Vec<String> = Vec::new();
            for arg in env::args(){
                args.push(arg);
            }
            self._args = args;
        }
        return self;
    }

    // 参数解析
    pub fn _parse_args(&mut self){
        for i in 0..self._args.len(){
            if i == 0{
                continue;
            }
            let arg = &self._args[i];
            // 去掉空格
            let arg = &String::from(arg.trim());
            // 第一个参数为命令
            if i == 1{
                if &arg[..1] != "-"{
                    self.command = arg.to_owned();
                    continue;
                }
            }
        }
    }

    // 运行
    pub fn run(&mut self){
        self._init_args();
        self._parse_args();
    }
}


