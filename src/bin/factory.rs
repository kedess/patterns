/**
 * Предоставляет интерфейс для создания семейств взаимосвязанных или взаимозависимых объектов, не специфийируя их конкретныъ классов
 * 
 * Применимость:
 * когда система не должна зависеть от того, как создаюся, компонуются и представляются входищие в нее объекты
 * когда входящие в семейство взаимосвязанные объекты должны использоваться вместе и вам необходимо обеспечить выполнение этого ограничения
 * когда система должна конфигурировать одним из семейств составляющих ее объектов
 * когда вы хотите предоставить библиотеку объектов, раскрывая только их интерфейсы, но не реализацию
**/

trait Button {
    fn click(&self);
}
struct ButtonQT{}
impl Button for ButtonQT {
    fn click(&self) {
        println!("Clicked QT button");
    }
}
struct ButtonGTK{}
impl Button for ButtonGTK {
    fn click(&self) {
        println!("Clicked GTK button");
    }
}
trait WidgetFactory {
    fn button(&self) -> Box<dyn Button>;
}
struct QtWidgetFactory{}
impl WidgetFactory for QtWidgetFactory {
    fn button(&self) -> Box<dyn Button> {
        Box::new(ButtonQT{})
    }
}
struct GtkWidgetFactory{}
impl WidgetFactory for GtkWidgetFactory {
    fn button(&self) -> Box<dyn Button> {
        Box::new(ButtonGTK{})
    }
}
fn main(){
    let factory = GtkWidgetFactory{};
    let button = factory.button();
    button.click();
}