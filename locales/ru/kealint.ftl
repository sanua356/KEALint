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
