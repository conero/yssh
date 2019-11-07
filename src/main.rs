//2019年11月1日 星期五
//uysh 项目主入口

//创建项目根
extern crate uysh;

fn main() {
    println!("uysh {}", uysh::START_DATE);
    println!("uymas {}", uymas::START_DATE);
    // 数据测试
    uymas::cmd::Cmd::new();
}
