/**
 * В объектно ориентированном программировании фабрикой называется объект,
 * создающий другие объекты. Формально фабрика — это функция или метод,
 * возвращающая объекты разных прототипов или классов из вызова какого-то
 * метода, который считается новым.
 *
 * Простая фабрика просто генерирует экземпляр для клиента без предоставления
 * какой-либо логики экземпляра.
 *
 * Когда использовать:
 * - когда создание объекта подразумевает какую-то логику, а не просто
 *   несколько присваиваний, то имеет смысл делегировать задачу выделенной
 *   фабрике, а не повторять повсюду один и тот же код.
**/

#[derive(Debug)]
#[allow(unused)]
enum Color {
    White,
    Black,
}

#[derive(Debug)]
#[allow(unused)]
struct Door {
    name: String,
    color: Color,
}

impl Door {
    fn new(name: &str, color: Color) -> Self {
        Door {
            name: name.to_string(),
            color,
        }
    }
}

struct DoorFactory {}

impl DoorFactory {
    fn make(name: &str, color: Color) -> Option<Door> {
        /*
         * Какая та дополнительная логика, которую не стоит раскрывать или
         * дополнительные проверки
         */
        if name.len() > 10 {
            return None;
        }
        Some(Door::new(name, color))
    }
}

fn main() {
    let door = DoorFactory::make("Wood Door", Color::White).unwrap();
    println!("{:?}", door);
}
