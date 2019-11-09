//2019年11月1日 星期五
//uysh 项目主入口

//创建项目根
extern crate uysh;

fn main() {
    // 数据测试
    let mut cmd = uymas::cmd::Cmd::new();

    // 项目测试
    //cmd.set_args(vec!["test", "name"]);

    cmd.run();
}
