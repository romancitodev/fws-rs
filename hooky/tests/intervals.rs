use hooky::watcher;

#[cfg(test)]
mod tests {
    use super::watcher::{Errors, Interval};
    use std::time::Duration;
    #[test]
    fn test_good_parsing() {
        let interval = "500ms".parse::<Interval>();
        assert_eq!(interval, Ok(Interval::Check(Duration::from_millis(500))))
    }

    #[test]
    fn test_bad_parsing() {
        let seconds = "500h".parse::<Interval>();
        assert_eq!(seconds, Err(Errors::InvalidParseError))
    }

    #[test]
    fn test_no_expression() {
        let seconds = "500".parse::<Interval>();
        assert_eq!(seconds, Err(Errors::InvalidParseError))
    }

    #[test]
    fn test_no_time() {
        let seconds = "s".parse::<Interval>();
        assert_eq!(seconds, Err(Errors::InvalidParseError))
    }

    #[test]
    fn test_milli_seconds() {
        let seconds = "500ms".parse::<Interval>().unwrap();
        assert_eq!(seconds, Interval::Check(Duration::from_millis(500)))
    }

    #[test]
    fn test_seconds() {
        let seconds = "30s".parse::<Interval>().unwrap();
        assert_eq!(seconds, Interval::Check(Duration::from_secs(30)))
    }

    #[test]
    fn test_minutes() {
        let minutes = "2m".parse::<Interval>().unwrap();
        assert_eq!(minutes, Interval::Check(Duration::from_secs(120)))
    }
}
