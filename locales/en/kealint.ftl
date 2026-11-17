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
