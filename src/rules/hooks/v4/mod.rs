#[cfg(test)]
pub mod _tests;

pub mod bad_hooks_order_v4;
pub mod more_one_object_config_ha_v4;
pub mod multithread_modes_not_equal;
pub mod no_activated_host_cache_hook_for_radius_hook;
pub mod no_activated_host_cmds_for_database_backend;
pub mod no_basic_http_auth_in_ha_peers;
pub mod no_match_client_id_for_flex_id_hook;
pub mod unnecessary_activated_database_hooks;
pub mod use_user_check_hook;
