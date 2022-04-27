/* #[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_too() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_outside() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}
 */
mod base58_test;
mod base64_test;
