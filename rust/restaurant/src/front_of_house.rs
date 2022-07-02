// 默认私有的，添加 pub 才能变成公有的
pub mod hosting;

pub mod serving {
    fn take_order() {}

    pub fn server_order() {}

    fn take_payment() {}
}
