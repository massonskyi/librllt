#[cfg(test)]
mod tests {
    use crate::rllt::fabric::Fabric;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_add_and_execute_callback() {
        let mut fabric = Fabric::new();
        let flag = Arc::new(AtomicBool::new(false));

        let flag_clone = flag.clone();
        fabric.add_callback("test".to_string(), move || {
            flag_clone.store(true, Ordering::SeqCst);
        });

        fabric.execute_callback("test");
        assert!(flag.load(Ordering::SeqCst));
    }

    #[test]
    fn test_add_and_execute_callback_with_args() {
        let mut fabric = Fabric::new();
        fabric.add_callback_with_args("test_with_args".to_string(), |x: &i32| -> i32 {
            x + 1
        });

        let result = fabric.execute_callback_with_args("test_with_args", 10);
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_execute_all_callbacks() {
        let mut fabric = Fabric::new();
        let counter = Arc::new(AtomicBool::new(false));

        let counter_clone = counter.clone();
        fabric.add_callback("increment".to_string(), move || {
            counter_clone.store(true, Ordering::SeqCst);
        });

        let counter_clone2 = counter.clone();
        fabric.add_callback("increment_again".to_string(), move || {
            counter_clone2.store(true, Ordering::SeqCst);
        });

        fabric.execute();
        assert!(counter.load(Ordering::SeqCst));
    }

    #[test]
    fn test_remove_callback() {
        let mut fabric = Fabric::new();
        let flag = Arc::new(AtomicBool::new(false));

        let flag_clone = flag.clone();
        fabric.add_callback("test".to_string(), move || {
            flag_clone.store(true, Ordering::SeqCst);
        });

        fabric.remove_callback("test");
        fabric.execute_callback("test");
        assert!(!flag.load(Ordering::SeqCst));
    }

    #[test]
    fn test_execute_callback_with_wrong_type() {
        let mut fabric = Fabric::new();
        fabric.add_callback_with_args("test_with_args".to_string(), |x: &i32| -> i32 {
            x + 1
        });

        let result: Option<String> = fabric.execute_callback_with_args("test_with_args", 10);
        assert!(result.is_none());
    }

    #[test]
    fn test_add_and_execute_multiple_callbacks_with_args() {
        let mut fabric = Fabric::new();
        fabric.add_callback_with_args("callback1".to_string(), |x: &i32| -> i32 {
            x + 1
        });
        fabric.add_callback_with_args("callback2".to_string(), |s: &String| -> i32 {
            s.len() as i32
        });

        // Debug print to check registered callbacks
        println!("Fabric details: {:?}", fabric);

        let result1 = fabric.execute_callback_with_args("callback1", 10);
        let result2 = fabric.execute_callback_with_args("callback2", "Hello".to_string());

        // Debug print to check results
        println!("Result of callback2: {:?}", result2);

        assert_eq!(result1, Some(11));
        assert_eq!(result2, Some(5));
    }
}
