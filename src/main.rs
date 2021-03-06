//2019年11月1日 星期五
//yssh 项目主入口

//创建项目根
extern crate yssh;

use uymas::cmd::Cmd;

fn main() {
    // 数据测试
    let mut cmd = uymas::cmd::Cmd::new();

    // 项目测试
    // cmd.set_args(vec!["--test='name'"]);
    //cmd.set_args(vec!["scan", "--dir", "C:/conero/application"]);
    cmd.cmd_fn_call(&"version", cmd_version);
    cmd.cmd_fn_calls(vec!["help", "?"], cmd_help);
    cmd.cmd_fn_call(&"scan", cmd_scan);
    cmd.empty_fn_call(cmd_empty);
    cmd.run();
}

// 项目其实
fn cmd_empty(_app: &Cmd) {
    println!(
        " 欢迎使用 {}, power by {}.",
        yssh::PROJECT_CODE,
        yssh::AUTHOR
    );
    println!(" 命令行: [command] [option]");
    println!(" 输入 help 键入更多帮助信息");
}

//版本信息
fn cmd_version(_app: &Cmd) {
    println!("{}/{}", yssh::VERSION, yssh::RELEASE);
}

// 帮助信息
fn cmd_help(_app: &Cmd) {
    println!(" 命令列表如下");
    println!(" . version 版本信息打印");
    println!(" . scan 目录扫描工具");
    println!("   --dir=<target_dir> 目标目录，默认为 ./");
}

//目录扫描
fn cmd_scan(app: &Cmd) {
    let dir = app.raw_def(String::from("dir"), String::from("./"));
    //yssh::scan::dirs(dir);

    let mut di = yssh::scan::DirInfo::new(dir);
    di.scan_base();

    println!(" 正在读取目录: {}", di.path_dir);
    println!(
        " 统计数: {}, 文件为: {}, 目录: {}.",
        di.count, di.count_file, di.count_dir
    );
    println!(" 总大小: {}.", di.bytes);
    println!(" 用时时长: {}s.", di.run_second);
}
