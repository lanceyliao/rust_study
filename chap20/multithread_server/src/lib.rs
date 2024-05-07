use std::thread;

// pub struct ThreadPool;
pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// 创建线程池
    /// 
    /// 线程池中线程的数量
    /// 
    /// # Panics
    /// 
    /// `new` 函数在 size 为 0 时会 panic
    
    // 使用assert!宏来断言size大于0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
    // 返回Result类型，而不是直接panic
    // pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {

        let mut threads = Vec::with_capacity(size);
        // 与Vec::new有些类似，但区别在于with_capacity会为动态数组预分配出指定的空间。
        // 在知晓存储大小的前提下预先分配存储空间要比使用Vec::new在插入时动态扩展大小更有效率一些。
        for _ in 0..size {
            // 创建线程并将他们存储至动态数组中
        }
        ThreadPool {
            threads
        }
    }

    // thread::spawn的函数签名
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // where
    //     F: FnOnce() -> T + Send + 'static,
    //     T: Send + 'static,

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        // FnOnce后的()意味着传入的闭包既没有参数，也不返回结果。
        // 就像函数定义一样，我们可以省略签名中的返回值，但却不能省略函数名后的圆括号，即便括号中没有任何参数。
    {
    }
}