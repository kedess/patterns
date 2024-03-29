use std::collections::HashMap;

/**
 * Прототип (Prototype) — это порождающий паттерн проектирования, который позволяет копировать объекты, не вдаваясь в подробности их реализации.
 * Прототип поручает создание копий самим копируемым объектам. Для этого в базовый класс добавляется метод клонироания,
 * который создает новый объект на основе существующего. Паттерн вводит общий интерфейс для всех объектов, поддерживающих клонирование.
 * Это позволяет копировать объекты, не привязываясь к их конкретным классам.
 * * Когда использовать:
 * - Когда ваш код не должен зависеть от классов копируемых объектов
 * - Когда вы имеете уйму подклассов, которые отличаются начальными значениями полей. Кто-то мог создать все эти
 * классы, чтобы иметь возможность легко порождать объекты с определённой конфигурацией
 **/

#[allow(unused)]
struct Warrior {
    name: String,
    force: u32,
}

impl Clone for Warrior {
    fn clone(&self) -> Self {
        Warrior {
            name: self.name.clone(),
            force: 90,
        }
    }
}

fn main() {
    let mut warriors_dict = HashMap::new();
    warriors_dict.insert(
        "Knight",
        Warrior {
            name: "Knight".to_string(),
            force: 90,
        },
    );
    warriors_dict.insert(
        "Farmer",
        Warrior {
            name: "Farmer".to_string(),
            force: 40,
        },
    );
    // Мы можем заранее создать группу объектов как нам нужно, а потом вызывающий код будет их просто клонировать
    // Создадим армию из 5 рыцарей и 5 крестьян
    let mut warriors = vec![];
    for _ in 0..5 {
        warriors.push(warriors_dict.get("Knight").unwrap().clone());
        warriors.push(warriors_dict.get("Farmer").unwrap().clone());
    }
}
