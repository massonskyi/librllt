use std::any::Any;
#[cfg(test)]
mod tests {

    define!(TEST_U32_CONST: u32, 42);
    define!(TEST_F64_CONST: f64, 3.14);
    define!(TEST_STR_CONST: &'static str, "Hello, World!");

    define!(TEST_FUNCTION, {
        let x = 10;
        let y = 20;
        x + y
    });


    define!(TEST_STRUCT, MyStruct { value: u32 = 42 });

    #[test]
    fn test_consts() {
        assert_eq!(TEST_U32_CONST, 42);
        assert_eq!(TEST_F64_CONST, 3.14);
        assert_eq!(TEST_STR_CONST, "Hello, World!");
    }

    fn test_function() {
        let result = TEST_FUNCTION();
        assert_eq!(*result.downcast_ref::<i32>().unwrap(), 30);
    }


    #[test]
    fn test_struct() {
        assert_eq!(TEST_STRUCT.value, 42);
    }
}