#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::rllt::timeit::Timeit;
    #[test]
    fn test_timeit_struct() {
        let mut timer = Timeit::new();
        let start = timer.__enter();
        
        std::thread::sleep(Duration::from_secs(1)); // Simulate some time-consuming operation
        
        timer.__exit(start);
        println!("{}", timer.__str());
    }

    #[test]
    fn test_timeit_static() {
        Timeit::timeit(|| {
            std::thread::sleep(Duration::from_secs(1)); // Simulate some time-consuming operation
        });
    }
}