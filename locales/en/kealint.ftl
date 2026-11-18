problems-total = Found { $count } problem(s).
by-type-config = By type config:
by-importance = By importance:
problems = problem(s).
Critical = Critical
Warning = Warning
Info = Info

parsing-config-file-error = An error occurred while parsing the { $config_type } configuration: { $err }
reading-config-file-error = An error occurred while reading the { $config_type } configuration: { $err }
write-results-to-file-error = An error occurred when writing the test results to a file.
write-results-to-file-access-error = An error occurred while verifying the access rights of the output file: { $err }
convert-configuration-to-struct-error = An error occurred when converting the configuration to a structure.
convert-unix-socket-buffer-error = An error occurred when converting the socket buffer to a string.
sqlite-connection-error = An error occurred when connecting to an SQLite database.
save-results-to-database-error = An error occurred when saving check results in the database: { $err }
unix-socket-path-not-exists-error = The path for the UNIX listening socket is not specified!
path-to-database-not-exists-error = The path for the database is not specified!
database-migrations-applying-error = An error occurred when applying migrations in the database: { $err }
remove-previous-unix-socket-file-error = The previous UNIX socket file could not be deleted.
unix-socket-connection-error = An error occurred when connecting to a UNIX socket.
unix-socket-listening-error = Error in time listening UNIX socket: { $err }

write-results-to-file-successfully = Check results successfully written in file!
save-results-to-database = Check results saved to database successfully! { $datetime }
database-migrations-applied-successfully = Database migrations applied successfully!
standalone-mode-running-successfully = Server runned in standalone mode!

get_not_select_flq_allocator_in_global_level_config_rule = The 'FLQ' address allocator is not recommended for use at the global configuration level.
get_not_select_iterative_allocator_for_shared_lease_database = The 'iterative' address allocator is not recommended for use with a shared database of rents on several servers.

get_not_lifetime_for_additional_classes_rule = For the client class '{ $className }', the 'only-in-additional-list' flag is set to 'true'. The class contains any of the following parameters: 'min-valid-lifetime', 'valid-lifetime', 'max-valid-lifetime' or 'offer-lifetime', but they will have no effect.
get_not_recommended_prefix_AFTER__classes = The client class named '{ $className }' uses the prefix 'AFTER_'. This prefix is reserved by the system for a query processing mechanism that has not yet been written. It is recommended to replace the prefix with an arbitrary one.
evaluate_required_as_additional_classes_rule = The client class named '{ $className }' is specified as the value by the 'evaluate-additional-classes' key in the configuration, but does not have the 'only-if-required' or 'only-in-additional-list' flag set to 'true'.

get_template_rule = The socket handler '{ $socketType }' is not specified in the configuration for the 'control-sockets' key. Working with the API of this service may not be available.
no_all_control_sockets_specified_rule = The configuration does not specify the 'control-sockets' key with socket handlers. Working with the API may not be available.
not_local_ip_without_https_rule = The configuration specifies the 'http-port' key in a value that is not a local IP address, but HTTPS support is not enabled.

get_not_ddns_qualifying_suffix_with_enabled_ddns_updates_rule = It is recommended to specify the value for the 'ddns-qualifying-suffix' field when enabling DDNS updates.
not_local_ip_address_in_d2_server_config_rule = Loopback addresses must be used as the server address to avoid attacks with fake requests.

check_retry_interval = The value of the 'retry-interval' parameter in the configuration of the 'GSS-TSIG' hook is recommended to be set no more than 1/3 of the difference between the values of the 'tkey-lifetime' and 'rekey-interval' parameters.
check_rekey_percent = The value of the 'rekey-interval' parameter in the configuration of the 'GSS-TSIG' hook is recommended to be set in the range of 50-80% of the value of the 'tkey-lifetime' parameter.
no_credentials_cache_and_client_keytab_together_in_GSSTSIG_rule = It is not recommended to specify both the 'credentials-cache' and 'client-keytab' parameters in the configuration of the '{ $hookName }' hook. Use one of the two above.
get_bad_hooks_order_rule_1 = It is recommended to move the '{ $hook1 }' hook first in the list, otherwise the correct operation of the '{ $hook2 }' hook may be disrupted.
get_bad_hooks_order_rule_2 = It is recommended to move the '{ $hook1 }' hook last in the list to ensure that all higher-level hooks have already contributed to packet processing.
get_bad_hooks_order_rule_3 = For correct operation, the '{ $hook1 }' hook must be specified before the '{ $hook2 }' hook.
get_bad_hooks_order_rule_4 = For correct operation, the '{ $hook1 }' hook must be specified before the '{ $hook2 }' hook.
get_bad_hooks_order_rule_5 = For correct operation, the '{ $hook1 }' hook must be specified before the '{ $hook2 }' hook.
get_more_one_object_config_HA_rule = For the hook '{ $hookName }', the 'high-availability' key cannot contain more than one object in the array.
get_multithread_modes_not_equal_rule = The multithreading control flags in the global server configuration and the high availability hook configuration are not equal.
get_no_activated_host_cache_hook_for_radius_hook = The '{ $hook1 }' hook requires activation of the '{ $hook2 }' hook to work correctly.
get_no_activated_host_cmds_for_database_backend_rule = When using databases to store host reservations using the 'hosts-database' or 'hosts-databases' key, it is recommended to use the '{ $hookName }' hook to interact with them in a predictable way.
get_no_basic_http_auth_in_ha_peers_rule = The peer named '{ $peerName }' of the high availability hook lacks basic HTTP authentication.
get_unnecessary_activated_database_hooks_rule_1 = The MySQL support hook is specified in the configuration of the hooks, but it does not serve any functionality.
get_unnecessary_activated_database_hooks_rule_2 = The PostgreSQL support hook is specified in the configuration of the hooks, but it does not serve any functionality.
get_use_user_check_hook_rule = The '{ $hookName }' hook is outdated. It is recommended to use it only for educational purposes. Use the hosts reservations mechanisms of the global configuration instead of the hook.
no_match_client_id_for_flex_id_hook_rule = The 'replace-client-id' parameter is set to 'true' inside the '{ $hookName }' hook. For it to work, the 'match-client-id' parameter must be set to 'true' in the global configuration.

get_no_active_interfaces = No network interfaces are specified in the server configuration. Addresses will not be serviced.

get_lease_sanity_checks_enabled_for_not_memfile_backend_rule = The Sanity Checks mechanism is not implemented for rent databases other than 'memfile'.
get_no_enabled_persist_flag_rule = The 'persist' flag is not set to 'true' for the maintenance of the lease database in the 'memfile'.
get_not_change_stop_retry_exit_strategy_on_fail_rule = It is recommended to set the 'on-fail' parameter in the 'lease-database' configuration to 'stop-retry-exit' for the correct processing of leases in the production environment.

get_debug_loggers_rule = The logger named '{ $loggerName }' has severity 'DEBUG'. Change severity if you are using a production server.
get_no_percent_m_in_pattern_rule = The logger named '{ $loggerName }' by the key 'pattern' does not have the literals '%m' outside datetime. The log message will not be available without it.
get_no_linebreak_in_pattern_rule = The logger named '{ $loggerName }' by the key 'pattern' does not have the literals '\n'. Log messages will not be transferred to a new line.

get_kea_incomplete_bytes_options = The option with { $optionId } has the parameter 'csv-format' set to 'false', but the key 'data' has incomplete octets in value '{ $optionValue }'. It is recommended to always specify octets in pairs (for example: FF, A1, 22).
get_kea_managed_options = The option with name '{ $optionName }' and code '{ $optionCode }' is controlled by the KEA engine and is not recommended for manual setting.

get_no_enable_queue_and_multithreading_together_rule = The 'dhcp-queue-control' parameters will not work while multithreading mode is activated for KEA in the global configuration using the 'multi-threading' key.

get_reservations_out_of_pool_in_subnet = In the subnet '{ $subnet }', all reservations are registered outside of dynamic pools. It is possible to set the 'reservations-out-of-pool' flag to 'true' to improve performance.
get_reservations_out_of_pool_without_in_subnet_flag_1 = In the subnet '{ $subnet }'{ $subtext }, the 'reservations-in-subnet' parameter is set to 'false'. As long as the flag is set to 'false', the 'reservations-out-of-pool' parameter will have no effect.
get_reservations_out_of_pool_without_in_subnet_flag_2 = At the upper configuration levels, the 'reservations-in-subnet' parameter is set to 'false'. As long as the flag is set to 'false', the 'reservations-out-of-pool' parameter inside the subnet '{ $subnet }'{ $subtext } will have no effect.
get_global_resrvations_occupy_dynamic_pool_in_subnets = The global reservation with the IP address '{ $ip }' belongs to the dynamic pool '{ $pool }'. It is recommended to change the reservation address or pool size.
