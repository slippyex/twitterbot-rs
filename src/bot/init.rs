use passwords::PasswordGenerator;
use crate::storage;
use std::io::stdout;
use std::io::stdin;
use std::io::Write;
use crate::models::BotConfig;

fn get_user_input(label: &str) -> String {
    let mut s= String::new();
    print!("{}", label);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

fn get_master_password() -> String {
    let mut master_pwd = storage::read_file(&storage::assemble_bot_filepath("master.dat"));
    if master_pwd == "" {
        let pg = PasswordGenerator {
            length: 16,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: true,
            strict: true,
        };
        master_pwd = pg.generate_one().unwrap();
        storage::write_file(master_pwd.as_str(), &storage::assemble_bot_filepath("master.dat"));
    }
    master_pwd
}

fn edit_twitter_credentials(mut bot_config: BotConfig) {
    bot_config.twitter_consumer_key = get_user_input("Twitter Consumer Key: ");
    bot_config.twitter_consumer_secret = get_user_input("Twitter Consumer Secret: ");
    bot_config.twitter_access_key = get_user_input("Twitter Access Key: ");
    bot_config.twitter_token_secret = get_user_input("Twitter Token Secret: ");

    storage::persist_config_to_storage(bot_config);

}
///
/// Checks and validates twitter credentials
///
/// requests Twitter API access keys and persists
/// them into an encrypted file
///
pub fn check_config() -> bool {
    let master_password = get_master_password();
    if master_password == "" {
        edit_twitter_credentials(BotConfig::default());
    } else {
        let mut bot_config = storage::get_config_from_storage();
        if bot_config.twitter_consumer_key == "" {
            edit_twitter_credentials(bot_config);
        }
    }

    true
}