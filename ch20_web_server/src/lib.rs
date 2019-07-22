pub struct ThreadPool;

impl ThreadPool {
    /// Create a new thread pool.
    ///
    /// The `size` is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` will panic if `size` is 0.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}

#[cfg(test)]
mod thread_pool_tests {
    use super::ThreadPool;

    #[test]
    #[should_panic]
    fn panics_if_zero_pool_size() {
        ThreadPool::new(0);
    }

    #[test]
    fn successful_creation() {
        ThreadPool::new(4);
    }
}
