extern crate popflash_parser;
#[cfg(test)]

mod _get_rounds_won {
    use popflash_parser::utility;
    use popflash_parser::EXAMPLE_GAME_URL;

    #[tokio::test]
    async fn example_game() {
        let _body = utility::get_body_from_url(EXAMPLE_GAME_URL).await;
    }
}
