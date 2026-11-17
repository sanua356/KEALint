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