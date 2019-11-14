//2019年11月14日 星期四
//扫描命令

use std::fs;

// 循环遍历
pub fn dirs(vdir: String) {
    let paths = fs::read_dir(vdir).unwrap();

    for path in paths {
        //println!("{}", path.unwrap().path().display());
        let pb = path.unwrap().path();
        let mut path_name = String::from(pb.to_str().unwrap());
        path_name = path_name.replace("\\", "/");
        if pb.is_dir() {
            println!("{}", path_name);
            dirs(path_name);
        }
    }
}
