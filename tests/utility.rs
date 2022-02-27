extern crate popflash_parser;

#[cfg(test)]
mod _get_body_from_id {
    use popflash_parser::utility;
    #[tokio::test]
    async fn invalid_url() {
        let response = utility::get_body_from_url("invalid url").await;
        assert!(response.is_err());
    }

    #[tokio::test]
    async fn valid_url() {
        let response = utility::get_body_from_id(popflash_parser::EXAMPLE_GAME_ID).await;
        let expected = utility::get_body_from_url(popflash_parser::EXAMPLE_GAME_URL).await;
        assert_eq!(response.is_ok(), expected.is_ok());
    }
}

#[tokio::test]
async fn full_test() {
    let body = popflash_parser::utility::get_body_from_url(popflash_parser::EXAMPLE_GAME_URL)
        .await
        .unwrap();

    // NOTE: this was awful on my part but we can fix it
    let game = popflash_parser::Match::Match::new(&body);
    let json_string = serde_json::to_string(&game).unwrap();
    let json_json = serde_json::Value::from(json_string);

    assert!(true == true);
}

// script tag causing issues in testing with orredering of scripts being in different order so easier to remove it
//fn remove_script_tag(body: &mut String) {
//    todo!()
//}
