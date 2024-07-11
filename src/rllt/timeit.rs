use std::time::{Instant, Duration};
pub(crate) struct Timeit {
    t: Duration,
}

impl Timeit {
    pub(crate) fn new() -> Self {
        Timeit {
            t: Duration::default(),
        }
    }

    pub(crate) fn __enter(&mut self) -> Instant {
        Instant::now()
    }

    pub(crate) fn __exit(&mut self, start: Instant) {
        let elapsed = start.elapsed();
        self.t += elapsed;
    }

    pub(crate) fn __str(&self) -> String {
        format!("Elapsed time is {:.6} seconds", self.t.as_secs_f64())
    }

    pub(crate) fn timeit<F, R>(func: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start_time = Instant::now();
        let result = func();
        let elapsed_time = start_time.elapsed();

        println!(
            "Function '{}' executed in {:.4} seconds",
            std::any::type_name::<F>(),
            elapsed_time.as_secs_f64()
        );

        result
    }
}