/**
 * Заместитель — это структурный паттерн проектирования, который позволяет подставлять
 * вместо реальных объектов специальные объекты-заменители. Эти объекты перехватывают
 * вызовы к оригинальному объекту, позволяя сделать что-то до или после передачи вызова
 * оригиналу.
 * Когда применять:
 * - Ленивая инициализация (виртуальный прокси). Когда у вас есть тяжёлый объект,
 * грузящий данные из файловой системы или базы данных
 * - Защита доступа (защищающий прокси). Когда в программе есть разные типы пользователей,
 * и вам хочется защищать объект от неавторизованного доступа
 * - Локальный запуск сервиса (удалённый прокси). Когда настоящий сервисный объект
 * находится на удалённом сервере
 * - Логирование запросов (логирующий прокси). Когда требуется хранить историю обращений
 * к сервисному объекту
 * - Кеширование объектов («умная» ссылка). Когда нужно кешировать результаты запросов
 * клиентов и управлять их жизненным циклом
 *
**/

trait Storage {
    fn save(&self);
    fn remove(&self);
}

struct Database {}
impl Database {
    fn new() -> Self {
        Database {}
    }
}
impl Storage for Database {
    fn save(&self) {
        println!("save data");
    }
    fn remove(&self) {
        println!("remove data");
    }
}
struct DatabaseProxy {
    service: Box<dyn Storage>,
}
impl DatabaseProxy {
    fn new(service: Box<dyn Storage>) -> Self {
        DatabaseProxy { service }
    }
}
impl Storage for DatabaseProxy {
    fn save(&self) {
        println!("Do before");
        self.service.save();
        println!("Do after");
    }
    fn remove(&self) {
        println!("Do before");
        self.service.remove();
        println!("Do after");
    }
}
fn main() {
    let service = Box::new(Database::new());
    let proxy = DatabaseProxy::new(service);
    proxy.save();
    proxy.remove();
}
