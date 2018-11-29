extern crate egg_mode;
extern crate futures;
extern crate tokio;
extern crate tokio_core;

use crate::storage::{get_config_from_storage, get_filters_from_storage};

use self::tokio_core::reactor;
use self::tokio_core::reactor::Core;
use self::egg_mode::search;
use self::egg_mode::search::ResultType;

use crate::models::TwitterVerify;
use crate::models::BotConfig;

pub fn bot_invocation() {
    println!("bot invocation goes here");
    let _filters = get_filters_from_storage();
    let mut core = reactor::Core::new().unwrap();
    let _config = get_config_from_storage();

    let token = get_token(&_config);
    search_tweets(&mut core, &token);
}

pub fn verify_account(_config: &BotConfig) -> TwitterVerify {
    let mut core = reactor::Core::new().unwrap();
    let token = get_token(&_config);
    let handle = core.handle();
    let user = match core.run(egg_mode::verify_tokens(&token, &handle)) {
        Err(_err) => panic!("could not validate given Twitter tokens, please verify your input on next start... {}", _err),
        Ok(result) => {
            result
        }
    };

    println!("Hello {} and welcome to our Twitter bot", user.screen_name);
    TwitterVerify {
        id: user.id,
        screen_name: user.screen_name.clone()
    }
}

fn search_tweets(core: &mut Core, token: &egg_mode::Token) {

    let handle = core.handle();
    let user = core.run(egg_mode::verify_tokens(&token, &handle));

    let rustlang = core.run(egg_mode::user::show("rustlang", token, &handle)).unwrap();

    let search = core.run(search::search("rustlang")
        .result_type(ResultType::Recent)
        .call(&token, &handle))
        .unwrap();

    for tweet in &search.statuses {
        println!("(@{}) {}", tweet.user.as_ref().unwrap().screen_name, tweet.text);
    }
    println!("{:?}", rustlang);
}


fn get_token(_config: &BotConfig) -> egg_mode::Token {
    let con_token = egg_mode::KeyPair::new(_config.twitter_consumer_key.to_string(),
                                           _config.twitter_consumer_secret.to_string());
    let access_token = egg_mode::KeyPair::new(_config.twitter_access_key.to_string(),
                                              _config.twitter_token_secret.to_string());
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    token
}