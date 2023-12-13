use std::time::Duration;

#[macro_export]
macro_rules! timer {
    ($function:expr $(, $arg:expr)*) => {
        {
            let watch = std::time::Instant::now();
            let result = $function($($arg, )*);
            TimerResult {
                time: watch.elapsed(),
                result
            }
        }
    };
}

pub struct TimerResult<T> {
    pub time: Duration,
    pub result: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_works() {
        const RESULT: &str = "This is a result string";

        fn some_function() -> String {
            RESULT.to_owned()
        }

        let result = timer!(some_function);

        assert_eq!(&result.result, RESULT);
    }

    #[test]
    fn params_work() {
        fn perform_ops(a: i32, b: usize) -> u32 {
            if a.is_positive() {
                a as u32 + b as u32
            } else {
                b as u32
            }
        }

        let result = timer!(perform_ops, -2, 5);

        assert_eq!(result.result, 5);
    }
}
