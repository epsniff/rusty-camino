
pub mod index_canister;
pub mod local_shard;
pub mod new_index;
pub mod types;
use lazy_static::lazy_static;

lazy_static! {
    static ref INDEX_CANISTER: index_canister::IndexCanister = index_canister::IndexCanister::new(
        std::path::PathBuf::from("/Users/ajroetker/projects/rust-camino/canister"),
        types::CanisterSettings{
            base_path: std::path::PathBuf::from("/Users/ajroetker/projects/rust-camino/canister"),
            server_id: 1,
        })
        .expect("Failed to create the global HTTP client instance");
}

pub fn canister() -> &'static index_canister::IndexCanister {
    &*INDEX_CANISTER
}