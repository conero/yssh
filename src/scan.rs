//2019年11月14日 星期四
//扫描命令

use std::fs;
use std::time::SystemTime;

// 目录信息
pub struct DirInfo {
    pub count: usize,            // 统计所有文件、目录
    pub count_dir: usize,        // 目录统计数
    pub count_file: usize,       // 文件统计数
    pub bytes: f64,              // 总文件大小
    pub run_second: f64,         // 运行秒数统计
    pub path_dir: String,        // 当前的目录路径
    pub path_files: Vec<String>, //包含的文件列表
}

impl DirInfo {
    pub fn new(path_dir: String) -> DirInfo {
        DirInfo {
            count: 0,
            count_dir: 0,
            count_file: 0,
            bytes: 0.0,
            run_second: 0.0,
            path_dir,
            path_files: vec![],
        }
    }

    // 基础扫描
    pub fn scan_base(&mut self) {
        self.run_second = 0 as f64;
        let sy_time = SystemTime::now();
        self._scan_base(self.path_dir.clone());

        self.run_second = sy_time.elapsed().unwrap().as_secs() as f64;

        // 运行时间小于秒精确到下一单位
        if self.run_second <= (0 as f64) {
            self.run_second = (sy_time.elapsed().unwrap().as_millis()) as f64;
            self.run_second = self.run_second * 0.001;
        }
    }

    // 内部扫描运行
    fn _scan_base(&mut self, vdir: String) {
        let paths = fs::read_dir(vdir).unwrap();

        for path in paths {
            self.count = self.count + 1;
            let pb = path.unwrap().path();
            let mut path_name = String::from(pb.to_str().unwrap());
            path_name = path_name.replace("\\", "/");
            if pb.is_dir() {
                self.count_dir = self.count_dir + 1;
                //println!("{}", path_name);
                self._scan_base(path_name);
            } else {
                self.count_file = self.count_file + 1;
            }
        }
    }
}

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
