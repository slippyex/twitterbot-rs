extern crate egg_mode;
extern crate futures;
extern crate tokio;
extern crate tokio_core;

use crate::storage::{get_config_from_storage, get_json_from_storage, persist_json_to_storage};

use self::egg_mode::search;
use self::egg_mode::search::ResultType;
use self::egg_mode::tweet;

use self::tokio_core::reactor;
use self::tokio_core::reactor::Core;

use crate::models::BotConfig;
use crate::models::FilterRule;
use crate::models::TwitterVerify;
use crate::models::BotTweet;

pub fn bot_invocation() {
    info!("bot invocation triggered");
    let _filters = match get_json_from_storage::<Vec<FilterRule>>("filter_rules.json") {
        Some(filters) => filters,
        None => Vec::new(),
    };
    let mut core = reactor::Core::new().unwrap();
    let _config = get_config_from_storage();

    let token = get_token(&_config);

    let mut already_retweeted = match get_json_from_storage::<Vec<BotTweet>>("retweeted.json") {
        Some(retweets) => retweets,
        None => Vec::new(),
    };

    for filter in _filters.iter() {
        info!("Iterating over filter {:?}", filter.query);
        search_tweets(filter, &mut core, &token, &mut already_retweeted);
    }
    persist_json_to_storage(already_retweeted.to_vec(), "retweeted.json");
}

pub fn verify_account(_config: &BotConfig) -> TwitterVerify {
    let mut core = reactor::Core::new().unwrap();
    let token = get_token(&_config);
    let handle = core.handle();
    let user = match core.run(egg_mode::verify_tokens(&token, &handle)) {
        Err(_err) => panic!(
            "could not validate given Twitter tokens, please verify your input on next start... {}",
            _err
        ),
        Ok(result) => result,
    };

    println!("Hello {} and welcome to our Twitter bot", user.screen_name);
    TwitterVerify {
        id: user.id,
        screen_name: user.screen_name.clone(),
    }
}

fn search_tweets(filter: &FilterRule, core: &mut Core, token: &egg_mode::Token, already_retweeted: &mut Vec<BotTweet>) {
    let handle = core.handle();

    let result_type = match filter.query.result_type.as_ref() {
        "mixed" => ResultType::Mixed,
        "popular" => ResultType::Popular,
        _ => ResultType::Recent,
    };

    let search = core
        .run(
            search::search(filter.query.q.clone())
                .result_type(result_type)
                .count(filter.query.count)
                .call(&token, &handle),
        )
        .unwrap();
    for tweet in &search.statuses {
        
        if !already_retweeted.iter().any(|x| x.id == tweet.id) {
            let retweet_result = match core.run(tweet::retweet(tweet.id, &token, &handle)) {
                Ok(res) => {
                    BotTweet {
                        id: res.id,
                        text: res.text.clone(),
                        user_id: res.user.clone().unwrap().id,
                        filter_involved: filter.query.q.clone()
                    }
                },
                Err(_) => {
                    BotTweet {
                        id: tweet.id,
                        text: tweet.text.clone(),
                        user_id: tweet.user.clone().unwrap().id,
                        filter_involved: filter.query.q.clone()
                    }
                }
            };
            already_retweeted.push(retweet_result);
            println!(
                "(@{}) {}",
                tweet.user.as_ref().unwrap().screen_name,
                tweet.text
            );
        } else {
           info!("tweet id {} has already been retweeted before", tweet.id);
        }

    }
}

fn get_token(_config: &BotConfig) -> egg_mode::Token {
    let con_token = egg_mode::KeyPair::new(
        _config.twitter_consumer_key.to_string(),
        _config.twitter_consumer_secret.to_string(),
    );
    let access_token = egg_mode::KeyPair::new(
        _config.twitter_access_key.to_string(),
        _config.twitter_token_secret.to_string(),
    );
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    token
}
