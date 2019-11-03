//2019年11月1日 星期五
//uysh 项目主入口

mod lib;
use lib::variable;
fn main() {
    println!("Hello, world!");
    // @todo 此处 【variable::variable】 编写不优雅
    println!("{}", variable::variable::START_DATE);
}
