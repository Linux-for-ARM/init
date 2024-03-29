# lfa_init

lfa_init - простейшая система инициализации для LFA. Она предназначена для изучения принципа работы таких программ.

## Программы

- `/sbin/init` - система инициализации;
- `/sbin/service` - программа для управления сервисами (включение/выключение их загрузки, запуск, перезапуск и остановка);
- `/sbin/poweroff` - останавливает все запущенные сервисы и выключает систему;
- `/sbin/reboot` - останавливает все запущенные сервисы и перезагружает систему;

## Примитивы

lfa_init активно использует два понятия: *сервис* и *уровень запуска*.

## Сервисы

Сервис - это TOML-конфиг, содержащий команды для запуска, перезапуска и остановки системных компонентов и программ. Упрощённый аналог *загрузочных скриптов* из SysVInit и OpenRC.

## Уровни запуска

Как и во многих других системах инициализации, в lfa_init есть понятие уровней запуска (runlevel). Runlevel - режим функционирования ОС, использующей ядро Linux, подразумевающий наличие в нём тех или иных функций. В lfa_init для каждого уровня запуска существует свой набор сервисов:

- `rl0` - остановка системы. Не содержит никаких сервисов;
- `rl1` - загрузка системы в однопользовательском режиме. Смонтированы указанные в `/etc/fstab` разделы, но ни один сервис не загружен, в системе присутствует только пользователь `root`;
- `rl2` - многопользовательский режим **без** поддержки сети (сетевые сервисы находятся в следующем уровне запуска);
- `rl3` - многопользовательский режим с поддержкой сети. Обычно этого уровня хватает для работы серверов;
- `rl4` - многопользовательский режим с поддержкой графики (Xorg, Wayland). В LFA на данный момент отсутствует;
- `rl5` - перезагрузка системы. Не содержит никаких сервисов;

## Сравнение систем инициализации

| Функция | LFA init | sysvinit | OpenRC | systemd |
|---------|----------|--------|---------|----------|
| Поддерживаемые системы | Linux (LFA only) | Linux/BSD | Linux/BSD | Linux |
| Главный ЯП | Rust | C | Shell + C | C |
| Формат загрузочного скрипта/сервиса | TOML-конфиг | sh-скрипт | sh-скрипт | ini-конфиг |
| Конфигурация для каждого сервиса | Планируется в версии 1.1 | Нет | Да | Да |
| Лицензия | MIT | GPL v2+ | 2-cl. BSD | LGPL v2.1+ |



