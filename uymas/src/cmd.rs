use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fmt::{Error, Formatter};

//测试文件
//@todo 失败，且 intellJ rust plugin 无效
//#[path("./cmd_test.rs")]
//#[cfg(test)] mod cmd_test;

// 选项集合:
//      解析规则: (-xyz, --xyz, --test, --any) => [x, y, z, xyz, test, any]
// 原始数据
//      解析规则: --key=value, "key=value", "key=value&key2=value"
pub struct Cmd {
    pub command: String,                      // 命令
    _args: Vec<String>,                       // 内R部参数，默认由系统参数读取
    _setting: Vec<String>,                    // 选项集合
    _data_raw: HashMap<String, String>,       // 原始输入值
    _cf_empty: fn(app: &Cmd),                 // 空的函数调用
    _cf_none: fn(app: &Cmd, _cmd: &str),      // 空的函数调用
    _cf_cmds: HashMap<String, fn(app: &Cmd)>, // 初始化方法
}

// 接口应用
pub struct App {
    pub name: String,    // 描述
    pub command: String, // 命令
}

// 继承 base 方法
pub trait BaseApp {
    fn index(&self);
}

// 默认空调用方法
fn _cf_empty(_app: &Cmd) {
    println!(" 欢迎使用 uymas 命令行解析工具")
}

// 方法不存在的调用
fn _cf_none(_app: &Cmd, _cmd: &str) {
    println!("{} 命令不存在！", _cmd)
}

//字符串分割为kv型
pub fn split_str_kv(ss: String, s: char) -> (i8, [String; 2]) {
    let mut kv: [String; 2] = [String::new(), String::new()];
    let mut idx: i8 = -1;
    let ss_to_char = ss.chars();
    let mut i: i8 = 0;
    for c in ss_to_char {
        if c == s {
            idx = i;
            let tmp_i = i as usize;
            kv[0] = String::from(&ss[..tmp_i]);
            kv[1] = str_trim_quote(String::from(&ss[tmp_i + 1..]));
            break;
        }
        i = i + 1;
    }
    return (idx, kv);
}

// 清理字符串中的多的双引号
// 'string' => string
// "string" => string
pub fn str_trim_quote(mut raw_str: String) -> String {
    //let chars = raw_str.chars();
    let len = raw_str.len();
    let first = &raw_str[..1];
    let end = &raw_str[len - 1..];

    if first == end {
        if first == "'" || first == "\"" {
            let new_str = &raw_str[1..len - 1];
            raw_str = new_str.parse().unwrap()
        }
    }
    raw_str
}

// 命令行
impl Cmd {
    // 构造函数
    pub fn new() -> Cmd {
        Cmd {
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
            for arg in env::args() {
                args.push(arg);
            }
            self._args = args;
        }
        return self;
    }

    // 参数解析
    pub fn _parse_args(&mut self) {
        for i in 0..self._args.len() {
            if i == 0 {
                continue;
            }
            let arg = &self._args[i];
            // 去掉空格
            let arg = &String::from(arg.trim());
            // 第一个参数为命令
            if i == 1 {
                if &arg[..1] != "-" {
                    self.command = arg.to_owned();
                    continue;
                }
            }
            // 选项解析
            // (--|-)
            if &arg[..1] == "-" {
                // --
                if &arg[..2] == "--" {
                    let value = &arg[2..];
                    let (idx, dd) = split_str_kv(String::from(value), '=');
                    //let idx = value.index("=");
                    if idx > -1 {
                        let key = &dd[0];
                        let v_str = &dd[1];
                        self._data_raw
                            .insert(String::from(key), String::from(v_str));
                    } else {
                        self._setting.push(value.to_string());
                    }
                } else {
                    let value = &arg[1..];
                    for v in value.split("") {
                        self._setting.push(v.to_string());
                    }
                }
            }
        }
    }

    // 设置参数，用于非系统args参数的调用
    pub fn set_args(&mut self, args: Vec<&str>) -> &mut Cmd {
        let mut new_args: Vec<String> = Vec::new();
        new_args.push(String::new());

        // 数据更新
        for arg in args {
            new_args.push(String::from(arg));
        }

        self._args = new_args;
        return self;
    }

    // 空命令函数调用
    pub fn empty_fn_call(&mut self, _call: fn(app: &Cmd)) -> &mut Cmd {
        self._cf_empty = _call;
        return self;
    }
    // 为发现的访问
    pub fn none_fn_call(&mut self, _call: fn(app: &Cmd, _cmd: &str)) -> &mut Cmd {
        self._cf_none = _call;
        return self;
    }

    // 函数式方法调用
    pub fn cmd_fn_call(&mut self, _cmd: &str, _call: fn(app: &Cmd)) -> &mut Cmd {
        self._cf_cmds.insert(String::from(_cmd), _call);
        return self;
    }

    // 方法路由
    pub fn router(&mut self) {
        if self.command.is_empty() {
            let call = self._cf_empty;
            call(self);
        } else {
            if self._cf_cmds.contains_key(&self.command) {
                let cl2 = self._cf_cmds.get(&self.command);
                cl2.unwrap()(self);
            } else {
                let cl_none = self._cf_none;
                cl_none(self, &self.command);
            }
        }
    }

    // 运行
    pub fn run(&mut self) {
        self._init_args();
        self._parse_args();
        self.router()
    }

    // 获取解析以后的参数
    pub fn get_args(self) -> Vec<String> {
        self._args
    }
    // 获取解析后的数据
    pub fn get_raw_data(self) -> HashMap<String, String> {
        self._data_raw
    }
    // 带默认值运行
    pub fn raw_def(self, key: String, def: String) -> String {
        if self._data_raw.contains_key(key.as_str()) {
            return self._data_raw.get(key.as_str()).unwrap().to_string();
        }
        return def;
    }
}

// 打印、调试必须继承 fmt::Debug trait
// 调试输出
impl fmt::Debug for Cmd {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Cmd {{ _args: {:?}, _setting: {:?}, _data_raw: {:?}}}",
            self._args, self._setting, self._data_raw
        )
    }
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Cmd {{ _args: {:?}, _setting: {:?}, _data_raw: {:?}}}",
            self._args, self._setting, self._data_raw
        )
    }
}

//默认方法
impl App {
    // 构造函数
    pub fn new(name: String, command: String) -> App {
        App { name, command }
    }

    // 默认命令
    pub fn index(self) {
        unimplemented!()
    }
}

// 方法继承
impl BaseApp for App {
    fn index(&self) {
        println!("欢迎使用命令 {}!", self.command);
    }
}
