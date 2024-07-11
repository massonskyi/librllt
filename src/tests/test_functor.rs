// Type alias for convenience

type TestResult = String;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use crate::rllt::functor::{F, KwargsKey, KwargsValue};
    // Mock functions for testing
    fn add_one(args: Vec<i32>, _: HashMap<KwargsKey, KwargsValue>) -> TestResult {
        (args[0] + 1).to_string()
    }

    fn multiply(args: Vec<i32>, kwargs: HashMap<KwargsKey, KwargsValue>) -> TestResult {
        (args[0] * kwargs.values().next().unwrap().parse::<i32>().unwrap()).to_string()
    }

    fn double(args: Vec<i32>, _: HashMap<KwargsKey, KwargsValue>) -> TestResult {
        (args[0] * 2).to_string()
    }

    fn square(args: Vec<i32>, _: HashMap<KwargsKey, KwargsValue>) -> TestResult {
        (args[0] * args[0]).to_string()
    }

    fn is_even(args: Vec<i32>, _: HashMap<KwargsKey, KwargsValue>) -> bool {
        args[0] % 2 == 0
    }

    // Test cases
    #[test]
    fn test_call() {
        let mut f = F::new(add_one, vec![1], HashMap::new());
        assert_eq!(f.call(vec![], HashMap::new()), "2");
    }

    #[test]
    fn test_chain() {
        let mut f = F::new(add_one, vec![1], HashMap::new());
        let mut chained = f.chain(double);
        assert_eq!(chained.call(vec![], HashMap::new()), "4");
    }

    #[test]
    fn test_map() {
        let mut f = F::new(add_one, vec![1], HashMap::new());
        let mut mapped = f.map(square);
        assert_eq!(mapped.call(vec![], HashMap::new()), "4");
    }

    #[test]
    fn test_filter() {
        let mut f = F::new(add_one, vec![1], HashMap::new());
        let filtered = f.filter(is_even);
        assert_eq!(filtered, None);
    }

    #[test]
    fn test_curry() {
        let mut f = F::new(add_one, vec![1], HashMap::new());
        let mut curried = f.curry(vec![], HashMap::new());
        assert_eq!(curried.call(vec![], HashMap::new()), "2");
    }

    #[test]
    fn test_transform_args() {
        let mut f = F::new(add_one, vec![1], HashMap::new());
        let mut transformed = f.transform_args(|args, _| (vec![args[0] * 2], HashMap::new()));
        assert_eq!(transformed.call(vec![], HashMap::new()), "4");
    }

    #[test]
    fn test_clear_cache() {
        let mut f = F::new(add_one, vec![1], HashMap::new());
        f.call(vec![], HashMap::new());
        f.clear_cache();
        assert!(f.cache.is_empty());
    }
}