use std::collections::HashMap;
use std::any::Any;
use std::boxed::Box;
use std::fmt::{self, Debug};
pub struct Fabric {
    callbacks_void: HashMap<String, Box<dyn Fn() + 'static>>,
    callbacks_with_args: HashMap<String, Box<dyn Fn(&[Box<dyn Any>]) -> Box<dyn Any> + 'static>>,
}
impl Debug for Fabric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fabric")
            .field("callbacks_void", &self.callbacks_void.keys().collect::<Vec<_>>())
            .field("callbacks_with_args", &self.callbacks_with_args.keys().collect::<Vec<_>>())
            .finish()
    }
}
impl Fabric {
    pub fn new() -> Self {
        Fabric {
            callbacks_void: HashMap::new(),
            callbacks_with_args: HashMap::new(),
        }
    }

    pub fn add_callback<F>(&mut self, name: String, callback: F)
    where
        F: Fn() + 'static,
    {
        self.callbacks_void.insert(name, Box::new(callback));
    }

    pub fn add_callback_with_args<F, R, A>(&mut self, name: String, callback: F)
    where
        F: Fn(&A) -> R + 'static,
        R: 'static,
        A: 'static + Debug,
    {
        self.callbacks_with_args.insert(name, Box::new(move |args| {
            let arg = args[0].downcast_ref::<A>().expect("Failed to downcast argument");
            Box::new(callback(arg))
        }));
    }

    pub fn remove_callback(&mut self, name: &str) {
        self.callbacks_void.remove(name);
        self.callbacks_with_args.remove(name);
    }

    pub fn execute(&self) {
        for callback in self.callbacks_void.values() {
            callback();
        }
    }

    pub fn execute_callback(&self, name: &str) {
        if let Some(callback) = self.callbacks_void.get(name) {
            callback();
        }
    }

    pub fn execute_callback_with_args<R, A>(&self, name: &str, arg: A) -> Option<R>
    where
        R: 'static,
        A: 'static + Debug,
    {
        if let Some(callback) = self.callbacks_with_args.get(name) {
            let result = callback(&[Box::new(arg)]);
            return result.downcast::<R>().ok().map(|r| *r);
        }
        None
    }
}