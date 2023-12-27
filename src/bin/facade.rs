/**
 * Паттерн «Фасад» предоставляет унифицированный интерфейс вместо набора интерфейсов
 * некоторой подсистемы. Фасад определяет интерфейс более высокого уровня, который
 * упрощает использование подсистемы
 *
 * Когда использовать:
 * - Когда вам нужно представить простой или урезанный интерфейс к сложной подсистеме
 * - Когда вы хотите разложить подсистему на отдельные слои. Используйте фасады для определения
 * точек входа на каждый уровень подсистемы. Если подсистемы зависят друг от друга,
 * то зависимость можно упростить, разрешив подсистемам обмениваться информацией
 * только через фасады
**/

struct FacadeVideoFramework {}

impl FacadeVideoFramework {
    fn new() -> Self {
        FacadeVideoFramework {}
    }
    fn convert(&self, _filename: &str, _codec: &str) {
        // TODO: здесь вся сложная логика фрейморка для конвертации видео файла
        /*
         * Можно дегко сменить фреймворк илил библиотеку, так как внешний код не зависит от них
         */
    }
}

fn main() {
    let facade = FacadeVideoFramework::new();
    facade.convert("sample.ts", "h265");
}