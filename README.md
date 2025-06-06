# Algo

Репозиторий с решением лабораторных работ по алгоритмам

## Лабораторные и реализованные алгоритмы

Условия задач и описание решений вложены в README.md в каждой папке лабораторной работы.

1. [Введенеие в алгоритмы. Рекурсия](./lab1)
    - Пузырьковая сортировка([bubble_sort](./lab1/src/modules/sorting.rs))
    - Реализация [BigIng](./lab1/src/modules/big_int.rs) на основе массива
2. [Сортировка и поиск](./lab2)
    - Быстрая сортировка Хоара([quick_sort](./lab2/src/modules/quick_sort.rs))
    - Сортировка слиянием([merge_sort](./lab2/src/modules/merge_sort.rs))
    - Алгоритм Эндрю для нахождения минимальной выпуклой оболочки([Задача 5](./lab2/README.md#задача-5-оболочка))
    - Внешняя сортировка файла превышающего размер оперативной памяти([Задача 7](./lab2/README.md#задача-7-внешняя-сортировка))
3. [Элементарные структуры данных](./lab3)
    - Кастомная структура объединяющая словарь и кучу для получения наиболее приоритетных ключей([HashMaxHeap](./lab3/src/modules/hash_max_heap.rs))
    - Сортировка кучей([heap_sort](./lab3/src/modules/heap_sort.rs))
    - Бинарный поиск([Задача 10](./lab3/README.md#задача-10-вложенные-отрезки))
4. [Деревья поиска](./lab4)
    - [Дерево отрезков(сегментов)](./lab4/src/modules/segtree_clone.rs)
    - [Минимальная куча](./lab4/src/modules/heap.rs)
5. [Хэширование](./lab5)
    - Своя база данных типа ключ-значение([hash_database](./lab5/src/modules/hash_database.rs))
    - Реализация словаря([dictionary](./lab5/src/modules/dictionary.rs))
    - Полиномиальные хэши([Задача 10](./lab5/README.md#задача-10-привидение-ваня))
6. [Жадные алгоритмы и динамическое программирование](./lab6)
    - Разные логические задачи
    - Расстояние Левенштейна([Задача 11](./lab6/README.md#задача-11-расстояние-по-левенштейну))
7. [Алгоритмы на графах](./lab7)
    - Поиск в ширину([Задача 3](./lab7/README.md#задача-3-зелье))
    - Нахождение соседей клетки([Задача 5](./lab7/README.md#задача-5-коврики))
    - Алгоритм Беллмана-Форда([Задача 7](./lab7/README.md#задача-7-кратчайшие-пути))
    - Система непересекающихся множеств([Задача 10](./lab7/README.md#задача-10-план-электрификации))
8. [Алгоритмы на строках](./lab8)
    - Нахождение/создание палиндромов([Задача 5](./lab8/README.md#задача-5-палиндром-он-же-палиндром))
    - Z-функция([Задания 7](./lab8/README.md#задача-7-басня-о-строке))

## Автотесты и бенчмарки

Почти к каждому заданию прилагаются тесты, а к некоторым особо критичным к производительности решениям - бенчмарки. Для их запуска необходимо установить [компилятор Rust](https://www.rust-lang.org/tools/install).

Можно перейти в нужную папку и запустить тесты/бенчмарки только для нее:

```
cd lab1
cargo test task2
cargo test
cargo bench task2
cargo bench
```

Для каждой лабораторной работы указано какие тесты и бенчмарки можно в ней запустить.

Подробнее про cargo можно прочитать в [документации](https://doc.rust-lang.org/cargo/commands/cargo-test.html) или вызвать `cargo test --help`, `man cargo bench`