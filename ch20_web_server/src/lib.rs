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
