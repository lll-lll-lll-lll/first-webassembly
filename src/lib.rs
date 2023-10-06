#[no_mangle]
pub fn add(a:i32, b:i32) -> i32 {
    return a + b
}

#[no_mangle]
pub fn get_timestamp() -> f64 {
    return unsafe {
        date_now()
    }
}
extern  {
    fn date_now() ->f64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(1, 2);
        assert_eq!(result, 3);
    }
}
