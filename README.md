# Box в Rust

## Что такое `Box<T>`

**`Box<T>`** — **владеющий указатель** с размещением данных в **куче** (heap). Это **самый простой** smart pointer в Rust.

```rust
let b = Box::new(42);
```

- **`b`** — `Box<i32>`.
- **Данные** `42` — **в куче**.
- **Указатель** `b` — **на стеке**.

## Как устроен `Box<T>`

### Внутри — **одно** машинное слово

```
Стек:                    Куча:
+-----------------+      +----------+
| b: Box<i32>    ─┼─────>| 42       |
| (8 байт)        |      +----------+
+-----------------+
```

- **`Box<T>`** — **один** указатель (8 байт на 64-битной системе).
- **Размер** `Box<T>` **не зависит** от размера `T`.

### `Drop` для `Box`

При **уничтожении** `Box`:

1. Вызывается **деструктор** `T` (`drop`).
2. Память **возвращается** аллокатору.

```rust
{
    let b = Box::new(String::from("hello"));
}   // ← drop: String уничтожен, память освобождена
```

## Когда нужен `Box`

### 1. **Рекурсивные** типы

```rust
// ❌ Нельзя — бесконечный размер
enum List {
    Cons(i32, List),
    Nil,
}

// ✅ Можно — List содержит Box
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

**Проблема:** без `Box` размер `List` = **бесконечность**.

**С `Box`:** размер известен — **указатель**.

### Пример

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
```

### 2. **Большие** значения

```rust
// Перемещение 1 МБ по стеку — дорого
let arr = [0u8; 1_048_576];

// Перемещение Box<[u8; 1MB]> — одно слово (8 байт)
let boxed = Box::new([0u8; 1_048_576]);
```

**Плюс:** при **передаче** `Box` копируется **только** указатель.

### 3. **Type erasure** (trait object)

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle(f64);
struct Square(f64);

impl Shape for Circle { ... }
impl Shape for Square { ... }

let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle(10.0)),
    Box::new(Square(20.0)),
];

for s in shapes {
    println!("{}", s.area());
}
```

- **`Box<dyn Shape>`** — **толстый** указатель (data + vtable).
- **Разные** типы в **одной** коллекции.
- **Динамическая** диспетчеризация.

### 4. Возврат **неизвестного** размера

```rust
fn make_future() -> Box<dyn Future<Output = u32>> {
    Box::new(async { 42 })
}
```

**Проблема:** компилятор **не знает** размер **анонимного** future.

**Решение:** `Box<dyn Future>` — **фиксированный** размер.

## Сводная таблица

| Ситуация | `Box` нужен? |
|---|---|
| **Рекурсивный** тип | ✅ Да |
| **Большие** значения | ✅ Да |
| **Trait object** | ✅ Да |
| **Неизвестный** размер | ✅ Да |
| **Маленькое** значение | ❌ Нет |
| **`&T`** достаточно | ❌ Нет |

## `Box` vs другие указатели

| | `Box<T>` | `&T` | `Rc<T>` | `Arc<T>` |
|---|---|---|---|---|
| **Владение** | ✅ | ❌ | ✅ | ✅ |
| **Clone** | ❌ | ✅ `Copy` | ✅ | ✅ |
| **Потоки** | ✅ `Send` | ⚠️ | ❌ | ✅ |
| **Счётчик** | ❌ | ❌ | ✅ | ✅ |
| **Куча** | ✅ | — | ✅ | ✅ |

## Примеры использования

### 1. **Рекурсивное** дерево

```rust
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

let tree = Node {
    value: 1,
    left: Some(Box::new(Node { value: 2, left: None, right: None })),
    right: None,
};
```

### 2. **Trait object** в коллекции

```rust
trait Animal {
    fn sound(&self) -> &str;
}

struct Dog;
struct Cat;

impl Animal for Dog { fn sound(&self) -> &str { "Woof" } }
impl Animal for Cat { fn sound(&self) -> &str { "Meow" } }

let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];
```

### 3. **Большие** структуры

```rust
struct BigData {
    buffer: [u8; 1_000_000],
}

// Перемещение BigData — 1 МБ
let big = BigData { buffer: [0; 1_000_000] };

// Перемещение Box<BigData> — 8 байт
let boxed = Box::new(BigData { buffer: [0; 1_000_000] });
```

### 4. **FFI** с C

```rust
extern "C" {
    fn malloc(size: usize) -> *mut u8;
}

let ptr = unsafe { malloc(1024) };
// ...
```

**`Box`** может **заменить** `malloc` в безопасном коде.

## Разыменование `Box`

```rust
let b = Box::new(42);

println!("{}", *b);        // 42 — разыменование
println!("{}", b);         // 42 — авто-разыменование
println!("{}", b + 1);     // 43

let s = Box::new(String::from("hello"));
println!("{}", s.len());   // 5 — метод через Deref
```

**`Box<T>`** реализует **`Deref`** → можно **использовать** как `T`.

## `Box::leak`

```rust
let b = Box::new(String::from("hello"));
let leaked: &'static mut String = Box::leak(b);
```

- **Утечка** памяти → `'static`.
- **Память** **никогда** не **освободится**.
- **Применение:** глобальные **singleton**.

## Сводная таблица

| Аспект | Описание |
|---|---|
| **Что** | Владеющий указатель |
| **Размер** | 8 байт (одно слово) |
| **Данные** | В куче |
| **`Drop`** | Вызывает деструктор + освобождает память |
| **`Deref`** | ✅ |
| **`Clone`** | ❌ |
| **`Send`** | ✅ (если `T: Send`) |

## Сводная таблица применений

| Применение | Пример |
|---|---|
| **Рекурсия** | `enum List { Cons(i32, Box<List>) }` |
| **Большие данные** | `Box<[u8; 1MB]>` |
| **Trait object** | `Vec<Box<dyn Shape>>` |
| **Неизвестный размер** | `Box<dyn Future>` |
| **FFI** | `Box::into_raw` |
| **Глобальные данные** | `Box::leak` |

## Итог

- **`Box<T>`** — **владеющий** указатель на **кучу**.
- **Размер** — **одно** машинное слово.
- **`Drop`** — **деструктор** + **освобождение** памяти.
- **`Deref`** — можно **использовать** как `T`.
- **Когда нужен:**
  - **рекурсивные** типы;
  - **большие** значения;
  - **trait object** (`Box<dyn Trait>`);
  - **неизвестный** размер (`Box<dyn Future>`).
- **`Box::leak`** — **утечка** → `'static`.
- **В вашем примере:** `Vec<Box<dyn Shape>>` — **гетерогенная** коллекция с **динамической** диспетчеризацией.
- **Правило:** `Box` — когда нужно **владение** + **куча** + **фиксированный** размер указателя.
