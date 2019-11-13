//2019年11月1日 星期五
//uysh 项目主入口

//创建项目根
extern crate uysh;

fn main() {
    // 数据测试
    let mut cmd = uymas::cmd::Cmd::new();

    // 项目测试
    // cmd.set_args(vec!["--test='name'"]);
    cmd.cmd_fn_call(&"version", cmd_version);
    cmd.cmd_fn_call(&"help", cmd_help);
    cmd.empty_fn_call(cmd_empty);
    cmd.run();
}

// 项目其实
fn cmd_empty() {
    println!(
        " 欢迎使用 {}, power by {}.",
        uysh::PROJECT_CODE,
        uysh::AUTHOR
    );
    println!(" 命令行: [command] [option]");
    println!(" 输入 help 键入更多帮助信息");
}

//版本信息
fn cmd_version() {
    println!("{}/{}", uysh::VERSION, uysh::RELEASE);
}

// 帮助信息
fn cmd_help() {
    println!(" 命令列表如下");
    println!(" . version 版本信息打印");
}
