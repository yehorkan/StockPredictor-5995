Ось основний приклад обробки даних на Rust. Цей код включає створення структури даних, що має методи для обробки даних, і використання їх для обробки даних з масиву.

```rust
use std::fmt;

// Створюємо структуру
#[derive(Debug)]
struct Data {
    value: i32,
}

impl Data {
    // Метод для збільшення значення на одиницю
    fn increment(&mut self) {
        self.value += 1;
    }

    // Метод для зменшення значення на одиницю
    fn decrement(&mut self) {
        self.value -= 1;
    }

    // Метод для подвоєння значення
    fn double(&mut self) {
        self.value *= 2;
    }

    // Метод для вирахування половини значення
    fn halve(&mut self) {
        self.value /= 2;
    }
}

// Реалізуємо Display trait для нашої структури
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn process_data(data: &mut Data, command: &str) {
    match command {
        "increment" => data.increment(),
        "decrement" => data.decrement(),
        "double" => data.double(),
        "halve" => data.halve(),
        _ => println!("Invalid command!"),
    }
}

fn main() {
    let mut data_array = vec![
        Data { value: 1 },
        Data { value: 2 },
        Data { value: 3 },
        Data { value: 4 },
        Data { value: 5 },
    ];

    let commands = vec![
        "increment",
        "decrement",
        "double",
        "halve",
    ];

    for data in &mut data_array {
        for command in &commands {
            process_data(data, command);
        }
    }

    for data in &data_array {
        println!("{}", data);
    }
}
```

Цей код має менше 150 рядків, але він показує базову обробку даних у Rust. Ви можете розширити цей код, додавши більше команд обробки, створюючи більш складні структури даних, або додавши більше функцій для обробки масивів даних.