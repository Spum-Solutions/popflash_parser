extern crate popflash_parser;

mod _match_from_id {
    use popflash_parser::*;

    #[tokio::test]
    async fn valid_call() {
        match_from_id(EXAMPLE_GAME_URL);
        assert_eq!(1, 0 + 1);
    }
}
