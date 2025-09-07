#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = pack_dir::add(2, 2);
        assert_eq!(result, 4);
    }
}
