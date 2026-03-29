# route-engine-rs
[![Version](https://img.shields.io/crates/v/route-engine-rs)](https://crates.io/crates/route-engine-rs)
[![Repository](https://img.shields.io/badge/GitHub-route--engine--rs-181717?logo=github)](https://github.com/ibatanov/route-engine-rs)
[![License](https://img.shields.io/crates/l/route-engine-rs)](License)


Легковесная библиотека для поиска маршрутов в ориентированном графе с расширяемыми стратегиями и бизнес-ограничениями.

## Что внутри

- `graph` — базовый граф `Graph<N, E>` и `NodeId`.
- `algorithms` — Dijkstra:
  - `shortest_path(...)`
  - `shortest_path_with_constraint(...)`
- `strategies` — как считать стоимость пути:
  - `ByCost` (1 критерий)
  - `ByTwoCosts` (2 критерия, лексикографический приоритет)
- `constraints` — правила прохождения ребер:
  - `PathConstraint`
  - `AllowAll`
- `errors` — ошибки библиотеки.

## Быстрый старт

Запуск примера:

```bash
cargo run --example transit_cost
```

Пример с накопленным состоянием (ограничение на число плеч):

```bash
cargo run --example stateful_constraint_max_hops
```

## Как расширять

1. Добавить новую стратегию:
реализовать `PathStrategy<E>` и определить `State` + `Key`.

2. Добавить новое бизнес-правило:
реализовать `PathConstraint<N, E, St>` и проверять ребро через `EdgeContext`,
используя `prev_state`/`next_state` при необходимости.

3. Запустить поиск:
- без ограничений: `shortest_path(...)`
- с ограничениями: `shortest_path_with_constraint(...)`

## Тесты

```bash
cargo test
```
