use std::{env, fs, path::Path};

pub const DEFAULT_GRPC_TIMEOUT_SECONDS: u32 = 300;
pub const DEFAULT_TEST_UN: &str = "tester";
pub const DEFAULT_TEST_PW: &str = "123456";
pub const DEFAULT_BACKING_DB_NAME: &str = "treaty.db";
pub const USE_LOCAL_IP: bool = false;

#[doc(hidden)]
pub mod common_contract_setup;
#[doc(hidden)]
pub mod harness;
#[doc(hidden)]
pub mod runner;

pub fn get_test_temp_dir(test_name: &str) -> String {
    let dir = env::temp_dir();
    let tmp = dir.as_os_str().to_str().unwrap();
    let path = Path::new(&tmp).join("TREATY_TESTS").join(test_name);

    if path.exists() {
        fs::remove_dir_all(&path).unwrap();
    }

    fs::create_dir_all(&path).unwrap();

    return path.as_path().to_str().unwrap().to_string();
}
