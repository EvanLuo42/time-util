#[macro_use]
pub mod time;


#[cfg(test)]
mod tests {
    use crate::time::{Second, Time};

    #[test]
    fn time_creation() {
        let second = time!(1 => Second);
        assert_eq!(1, second.value);
        assert_eq!(Second, second.unit);
    }

    #[test]
    fn time_plus() {
        let second = time!(1 => Second);
        assert_eq!(2, (second + 1).value);
    }

    #[test]
    fn time_eq() {
    }
}
