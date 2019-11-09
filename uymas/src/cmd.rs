use std::env;
use std::collections::HashMap;

//测试文件
//@todo 失败，且 intellJ rust plugin 无效
//#[path("./cmd_test.rs")]
//#[cfg(test)] mod cmd_test;

// 选项集合:
//      解析规则: (-xyz, --xyz, --test, --any) => [x, y, z, xyz, test, any]
// 原始数据
//      解析规则: --key=value, "key=value", "key=value&key2=value"
pub struct Cmd{
//    args: Vec<str>
    pub command: String,                // 命令
    _args: Vec<String>,                 // 内部参数，默认由系统参数读取
    _setting: Vec<String>,              // 选项集合
    _data_raw: HashMap<String, String>, // 原始输入值
    _cf_empty: fn(),                     // 空的函数调用
    _cf_none: fn(_cmd: &str),            // 空的函数调用
    _cf_cmds: HashMap<String, fn()>// 初始化方法
}

// 默认空调用方法
fn _cf_empty(){
    println!(" 欢迎使用 uymas 命令行解析工具")
}

// 方法不存在的调用
fn _cf_none(_cmd: &str){
    println!("{} 命令不存在！", _cmd)
}

// 命令行
impl Cmd{
    // 构造函数
    pub fn new() -> Cmd{
        Cmd{
            command: String::from(""),
            _args: Vec::new(),
            _setting: Vec::new(),
            _data_raw: HashMap::new(),
            _cf_empty,
            _cf_none,
            _cf_cmds: HashMap::new(),
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

    // 设置参数，用于非系统args参数的调用
    pub fn set_args(&mut self, args: Vec<&str>) -> &mut Cmd{
        let mut new_args: Vec<String> = Vec::new();
        new_args.push(String::new());

        // 数据更新
        for arg in args{
            new_args.push(String::from(arg));
        }

        self._args = new_args;
        return self;
    }

    // 空命令函数调用
    pub fn empty_fn_call(&mut self, _call: fn()) -> &mut Cmd{
        self._cf_empty = _call;
        return self
    }
    // 为发现的访问
    pub fn none_fn_call(&mut self, _call: fn(_cmd: &str)) -> &mut Cmd{
        self._cf_none = _call;
        return self
    }

    // 函数式方法调用
    pub fn cmd_fn_call(&mut self, _cmd: &str, _call: fn()) -> &mut Cmd{
        self._cf_cmds.insert(String::from(_cmd), _call);
        return self
    }

    // 方法路由
    pub fn router(&mut self){
        if self.command.is_empty(){
            let call = self._cf_empty;
            call();
        }else{
            if self._cf_cmds.contains_key(&self.command){
                let cl2 = self._cf_cmds.get(&self.command);
                cl2.unwrap();
            }else {
                let cl_none = self._cf_none;
                cl_none(&self.command);
            }
        }
    }

    // 运行
    pub fn run(&mut self){
        self._init_args();
        self._parse_args();
        self.router()
    }
}


