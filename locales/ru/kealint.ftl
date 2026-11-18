problems-total = Найдено { $count } проблем(ы).
by-type-config = По типам конфигураций:
by-importance = По степени важности:
problems = проблем(ы).
Critical = Критика
Warning = Предупреждение
Info = Информация

parsing-config-file-error = Произошла ошибка при разборе файла конфигурации { $config_type }: { $err }
reading-config-file-error = Произошла ошибка при чтении файла конфигурации { $config_type }: { $err }
write-results-to-file-error = Произошла ошибка при записи результатов теста в файл.
write-results-to-file-access-error = Произошла ошибка при проверке прав доступа к выходному файлу: { $err }
convert-configuration-to-struct-error = Произошла ошибка при преобразовании конфигурации в структуру.
convert-unix-socket-buffer-error = Произошла ошибка при преобразовании буфера сокета в строку.
sqlite-connection-error = Произошла ошибка при подключении к базе данных SQLite.
save-results-to-database-error = Произошла ошибка при сохранении результатов проверки в базу данных.
unix-socket-path-not-exists-error = Путь к сокету прослушивания UNIX не указан!
path-to-database-not-exists-error = Путь к базе данных не указан!
database-migrations-applying-error = Произошла ошибка при применении миграций в базе данных: { $err }
remove-previous-unix-socket-file-error = Не удалось удалить предыдущий файл сокета UNIX.
unix-socket-connection-error = Произошла ошибка при подключении к сокету UNIX.
unix-socket-listening-error = Ошибка при прослушивании сокета UNIX: { $err }

write-results-to-file-successfully = Результаты проверки успешно записаны в файл!
save-results-to-database = Результаты проверки успешно сохранены в базу данных!
database-migrations-applied-successfully = Миграции базы данных успешно применены!
standalone-mode-running-successfully = Сервер запущен в автономном режиме!

get_not_select_flq_allocator_in_global_level_config_rule = Распределение адресов 'FLQ' не рекомендуется использовать на глобальном уровне конфигурации.
get_not_select_iterative_allocator_for_shared_lease_database = 'Итеративный' распределитель адресов не рекомендуется использовать с общей базой данных аренды на нескольких серверах.

get_not_lifetime_for_additional_classes_rule = Для клиентского класса '{ $className }' флаг 'only-in-additional-list' установлен в значение 'true'. Класс содержит один из следующих параметров: 'min-valid-lifetime', 'valid-lifetime', 'max-valid-lifetime' или 'offer-lifetime', но они не будут иметь никакого эффекта.
get_not_recommended_prefix_AFTER__classes = Клиентский класс с именем '{ $className }' использует префикс 'AFTER_'. Этот префикс зарезервирован системой для механизма обработки запросов, который еще не написан. Рекомендуется заменить префикс на произвольный.
evaluate_required_as_additional_classes_rule = Клиентский класс с именем '{ $className }' указан в качестве значения ключа 'evaluate-additional-classes' в конфигурации, но для него не установлен флаг 'only-if-required' или 'only-in-additional-list' со значением 'true'.

get_template_rule = Обработчик сокетов '{ $socketType }' не указан в конфигурации для ключа 'control-sockets'. Работа с API этого сервиса может быть недоступна.
no_all_control_sockets_specified_rule = В конфигурации не указан ключ 'control-sockets' с обработчиками сокетов. Работа с API может быть недоступна.
not_local_ip_without_https_rule = В конфигурации указан ключ http-port со значением, которое не является локальным IP-адресом, но поддержка HTTPS не включена.

get_not_ddns_qualifying_suffix_with_enabled_ddns_updates_rule = При включении обновлений DDNS рекомендуется указать значение для поля 'ddns-qualifying-suffix'.
not_local_ip_address_in_d2_server_config_rule = Для предотвращения атак с использованием поддельных запросов в качестве адреса сервера необходимо использовать адреса обратной связи.

check_retry_interval = Значение параметра retry-interval в конфигурации хука GSS-TSIG рекомендуется устанавливать не более чем на 1/3 разницы между значениями параметров tkey-lifetime и rekey-interval.
check_rekey_percent = Значение параметра 'rekey-interval' в конфигурации хука 'GSS-TSIG' рекомендуется устанавливать в диапазоне 50–80 % от значения параметра 'tkey-lifetime'.
no_credentials_cache_and_client_keytab_together_in_GSSTSIG_rule = Не рекомендуется указывать одновременно параметры 'credentials-cache' и 'client-keytab' в конфигурации хука '{ $hookName }'. Используйте один из двух указанных выше параметров.
get_bad_hooks_order_rule_1 = Рекомендуется размещать хук '{ $hook1 }' первым в списке, иначе корректная работа хука '{ $hook2 }' может быть нарушена.
get_bad_hooks_order_rule_2 = Рекомендуется размещать хук '{ $hook1 }' последним в списке, чтобы все хуки более высокого уровня уже внесли свой вклад в обработку пакетов.
get_bad_hooks_order_rule_3 = Для корректной работы хук '{ $hook1 }' должен быть указан перед хуком '{ $hook2 }'.
get_bad_hooks_order_rule_4 = Для корректной работы хук '{ $hook1 }' должен быть указан перед хуком '{ $hook2 }'.
get_bad_hooks_order_rule_5 = Для корректной работы хук '{ $hook1 }' должен быть указан перед хуком '{ $hook2 }'.
get_more_one_object_config_HA_rule = Для хука '{ $hookName }' ключ 'high-availability' не может содержать более одного объекта в массиве.
get_multithread_modes_not_equal_rule = Флаги управления многопоточностью в глобальной конфигурации сервера и в конфигурации хука высокой доступности не совпадают.
get_no_activated_host_cache_hook_for_radius_hook = Для корректной работы хука '{ $hook1 }' требуется активация хука '{ $hook2 }'.
get_no_activated_host_cmds_for_database_backend_rule = При использовании баз данных для хранения данных о резервировании хостов с помощью ключа 'hosts-database' или 'hosts-databases' рекомендуется использовать хук '{ $hookName }' для предсказуемого взаимодействия с ними.
get_no_basic_http_auth_in_ha_peers_rule = У узла с именем '{ $peerName }' в системе высокой доступности отсутствует базовая HTTP-аутентификация.
get_unnecessary_activated_database_hooks_rule_1 = В конфигурации хуков указан хук поддержки MySQL, но он не выполняет никаких функций.
get_unnecessary_activated_database_hooks_rule_2 = В конфигурации хуков указан хук поддержки PostgreSQL, но он не выполняет никаких функций.
get_use_user_check_hook_rule = Хук '{ $hookName }' устарел.  Рекомендуется использовать его только в образовательных целях.  Вместо хука используйте механизмы резервирования хостов в глобальной конфигурации.
no_match_client_id_for_flex_id_hook_rule = Параметр 'replace-client-id' имеет значение 'true' внутри хука '{ $hookName }'. Чтобы это работало, параметр 'match-client-id' должен иметь значение 'true' в глобальной конфигурации.

get_no_active_interfaces = В конфигурации сервера не указаны сетевые интерфейсы. Адреса не будут обслуживаться.

get_lease_sanity_checks_enabled_for_not_memfile_backend_rule = Механизм Sanity Checks не реализован для баз данных аренды, отличных от 'memfile'.
get_no_enabled_persist_flag_rule = Флаг 'persist' не установлен в значение 'true' для поддержки базы данных аренды в 'memfile'.
get_not_change_stop_retry_exit_strategy_on_fail_rule = Для корректной обработки аренды в рабочей среде рекомендуется установить для параметра 'on-fail' в конфигурации 'lease-database' значение 'stop-retry-exit'.

get_debug_loggers_rule = Логгер с именем '{ $loggerName }' имеет уровень серьезности 'DEBUG'. Измените уровень серьезности, если вы используете рабочий сервер.
get_no_percent_m_in_pattern_rule = В логгере с именем '{ $loggerName }' по ключу 'pattern' нет литералов '%m' вне datetime. Без них сообщение журнала будет недоступно.
get_no_linebreak_in_pattern_rule = В логгере с именем '{ $loggerName }' по ключу 'pattern' нет литералов '\n'. Сообщения журнала не будут переноситься на новую строку.

get_kea_incomplete_bytes_options = Для параметра { $optionId } с ключом 'csv-format' установлено значение 'false', но ключ 'data' содержит неполные октеты в значении '{ $optionValue }'. Рекомендуется всегда указывать октеты парами (например: FF, A1, 22).
get_kea_managed_options = Параметр с именем '{ $optionName }' и кодом '{ $optionCode }' управляется системой KEA и не рекомендуется для ручной настройки.
