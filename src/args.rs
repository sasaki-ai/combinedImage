//! args
//! 
//! arg结构体即其方法

/// `get_nth_arg`返回[`String`]的参数<br>
/// 根据传入的[`usize`]类型的值获取参数
fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

/// `Args`用于适配命令行参数
#[derive(Debug)]
pub struct Args {
    ///合并图1地址
    pub image_1: String,
    ///合并图2地址
    pub image_2: String,
    ///合并结果地址
    pub output: String,
}

impl Args {
    /// 实现创建`Args`结构体的方法,并在内部用[`get_nth_arg`]给属性赋值
    pub fn new() -> Self {
        Args {
            image_1: get_nth_arg(1),
            image_2: get_nth_arg(2),
            output: get_nth_arg(3),
        }
    }
}
