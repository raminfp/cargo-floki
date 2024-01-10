#[cfg(test)]
mod tests {

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    #[test]
    fn test_add() {
        let cal = add(2, 3);
        assert_eq!(cal, 5);
    }
}
