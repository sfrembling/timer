use std::time::Duration;

/// A simple macro for timing a function.
///
/// Can be called with just a function, or with optional arguments
/// to be passed into the given function.
///
/// Returns a [TimerResult], which contains the result of the function
/// and the time it took for the function to execute.
///
/// Example:
/// ```
/// use ttimer::timer;
///
/// fn do_something(s: &str) -> u32 {
///     s.chars().count() as u32
/// }
///
/// let x = timer!(do_something, "Hello, world!");
///
/// println!("Took {} ns to execute", x.time.as_nanos());
/// assert_eq!(x.result, 13);
/// ```
#[macro_export]
macro_rules! timer {
    ($function:expr $(, $arg:expr)*) => {
        {
            use $crate::TimerResult;
            let watch = std::time::Instant::now();
            let result = $function($($arg, )*);
            TimerResult {
                time: watch.elapsed(),
                result,
                name: stringify!($function).to_owned()
            }
        }
    };
}

/// The result of using the `timer!` macro.
#[derive(Debug, Clone)]
pub struct TimerResult<T> {
    /// The time it took for the function to execute.
    pub time: Duration,
    /// The result returned by the function's execution.
    pub result: T,
    /// The name of the executed function.
    pub name: String,
}

impl<T> TimerResult<T> {
    /// Quickly print the result of the timer to StdOut
    ///
    /// Prints in the format: `{name} done in {time} ms`
    ///
    /// Example:
    /// ```
    /// use ttimer::timer;
    ///
    /// fn do_something() -> i32 {
    ///     0 // pretend this is some interesting function
    /// }
    ///
    /// timer!(do_something).view();
    /// ```
    pub fn view(&self) {
        println!("{} done in {} ms", self.name, self.time.as_millis());
    }
}

#[cfg(test)]
mod tests {
    use super::timer;

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

    #[test]
    fn name_works() {
        fn does_something() {
            let _ = String::new();
        }

        let result = timer!(does_something);

        assert_eq!(&result.name, "does_something");
    }
}
