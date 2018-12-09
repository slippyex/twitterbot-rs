mod storage;

pub use self::storage::assemble_bot_filepath;
pub use self::storage::get_config_from_storage;
pub use self::storage::get_json_from_storage;
pub use self::storage::persist_config_to_storage;
pub use self::storage::persist_json_to_storage;

pub use self::storage::read_file;
pub use self::storage::write_file;
