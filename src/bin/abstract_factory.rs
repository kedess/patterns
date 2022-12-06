/**
 * Шаблон «Абстрактная фабрика» описывает способ инкапсулирования группы
 * индивидуальных фабрик, объединённых некой темой, без указания для них
 * конкретных классов.
 *
 * Это фабрика фабрик. То есть фабрика, группирующая индивидуальные, но
 * взаимосвязанные/взаимозависимые фабрики без указания для них конкретных
 * классов.
 *
 * Когда использовать:
 * - когда у вас есть взаимосвязи с не самой простой логикой создания.
 * - когда входящие в семейство взаимосвязанные объекты должны использоваться
 *   вместе и вам необходимо обеспечить выполнение этого ограничения
 * - когда система должна конфигурировать одним из семейств составляющих ее
 *   объектов
 * - когда вы хотите предоставить библиотеку объектов, раскрывая только их
 *   интерфейсы, но не реализацию
**/

trait Button {
    fn click(&self);
}
struct ButtonQT {}
impl Button for ButtonQT {
    fn click(&self) {
        println!("Clicked QT button");
    }
}
struct ButtonGTK {}
impl Button for ButtonGTK {
    fn click(&self) {
        println!("Clicked GTK button");
    }
}
trait WidgetFactory {
    fn button(&self) -> Box<dyn Button>;
}
struct QtWidgetFactory {}
impl WidgetFactory for QtWidgetFactory {
    fn button(&self) -> Box<dyn Button> {
        Box::new(ButtonQT {})
    }
}
struct GtkWidgetFactory {}
impl WidgetFactory for GtkWidgetFactory {
    fn button(&self) -> Box<dyn Button> {
        Box::new(ButtonGTK {})
    }
}
fn main() {
    let factory = GtkWidgetFactory {};
    let button = factory.button();
    button.click();
}
