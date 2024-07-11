use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt;
use std::ops::Fn;


#[derive(Debug, PartialEq, Eq, Hash)]
struct CacheKey(Vec<i32>, HashMap<String, String>);

// Functor struct
pub(crate) struct F<Functor, Args, Kwargs> {
    func: Functor,
    args: Args,
    kwargs: Kwargs,
    cache: HashMap<CacheKey, Result>,
}

// Type definitions for convenience
pub  type KwargsKey = String;
pub type KwargsValue = String;
pub type Result = String;

// Implementing F
impl<Functor, Args, Kwargs> F<Functor, Args, Kwargs>
where
    Functor: Fn(Vec<Args>, HashMap<KwargsKey, KwargsValue>) -> Result,
    Args: Clone + PartialEq + Hash + fmt::Debug,
    Kwargs: Clone + PartialEq + Hash + fmt::Debug,
{
    // Constructor
    pub fn new(func: Functor, args: Args, kwargs: Kwargs) -> Self {
        F {
            func,
            args,
            kwargs,
            cache: HashMap::new(),
        }
    }

    // Call method
    pub fn call(&mut self, args: Vec<Args>, kwargs: HashMap<KwargsKey, KwargsValue>) -> Result {
        let cache_key = CacheKey(args.clone(), kwargs.clone());

        if let Some(result) = self.cache.get(&cache_key) {
            result.clone()
        } else {
            let result = (self.func)(args.clone(), kwargs.clone());
            self.cache.insert(cache_key, result.clone());
            result
        }
    }


    // Chain method
    pub fn chain<Transform>(&mut self, other_func: Transform) -> Result
    where
        Transform: Fn(Vec<Args>, HashMap<KwargsKey, KwargsValue>) -> Result,
    {
        let result = self.call(self.args.clone(), self.kwargs.clone());
        F::new(other_func, self.args.clone(), self.kwargs.clone()).call(vec![result], HashMap::new())
    }

    // Map method
    pub fn map<Transform>(&mut self, transform_func: Transform) -> Result
    where
        Transform: Fn(Vec<Args>, HashMap<KwargsKey, KwargsValue>) -> Result,
    {
        let result = self.call(self.args.clone(), self.kwargs.clone());
        F::new(transform_func, self.args.clone(), self.kwargs.clone()).call(vec![result], HashMap::new())
    }

    // Filter method
    pub fn filter<Filter>(&mut self, filter_func: Filter) -> Option<Result>
    where
        Filter: Fn(Vec<Args>, HashMap<KwargsKey, KwargsValue>) -> bool,
    {
        if filter_func(self.args.clone(), self.kwargs.clone()) {
            Some(self.call(self.args.clone(), self.kwargs.clone()))
        } else {
            None
        }
    }

    // Curry method
    pub fn curry(&mut self, curry_args: Vec<Args>, curry_kwargs: HashMap<KwargsKey, KwargsValue>) -> Self {
        F {
            func: self.func.clone(),
            args: self.args.clone(),
            kwargs: self.kwargs.clone(),
            cache: self.cache.clone(),
        }
    }

    // Transform args method
    pub fn transform_args<Transform>(&mut self, transform_func: Transform) -> F<Transform, Args, Kwargs>
    where
        Transform: Fn(Vec<Args>, HashMap<KwargsKey, KwargsValue>) -> (Vec<Args>, HashMap<KwargsKey, KwargsValue>),
    {
        let (transformed_args, transformed_kwargs) = transform_func(self.args.clone(), self.kwargs.clone());
        F::new(transform_func, transformed_args, transformed_kwargs)
    }

    // Reduce method
    pub fn reduce<Reducer>(&mut self, reducer_func: Reducer, initial: Option<Result>) -> Self
    where
        Reducer: Fn(Vec<Args>, HashMap<KwargsKey, KwargsValue>, Option<Result>) -> Result,
    {
        F {
            func: self.func.clone(),
            args: self.args.clone(),
            kwargs: self.kwargs.clone(),
            cache: self.cache.clone(),
        }
    }

    // Log call method
    pub fn log_call(&self, args: Vec<Args>, kwargs: HashMap<KwargsKey, KwargsValue>, result: Result) {
        println!("Calling {:?} with args {:?} and kwargs {:?} resulted in {:?}", self.func, args, kwargs, result);
    }

    // Clear cache method
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl<Functor, Args, Kwargs> PartialEq for F<Functor, Args, Kwargs>
where
    Functor: Fn(Vec<Args>, HashMap<KwargsKey, KwargsValue>) -> Result,
    Args: Clone + PartialEq + Hash + fmt::Debug,
    Kwargs: Clone + PartialEq + Hash + fmt::Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.func == other.func && self.args == other.args && self.kwargs == other.kwargs
    }
}

impl<Functor, Args, Kwargs> Eq for F<Functor, Args, Kwargs>
where
    Functor: Fn(Vec<Args>, HashMap<KwargsKey, KwargsValue>) -> Result,
    Args: Clone + PartialEq + Hash + fmt::Debug,
    Kwargs: Clone + PartialEq + Hash + fmt::Debug,
{
}