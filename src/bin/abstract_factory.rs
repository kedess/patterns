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
trait Label {
    fn onfocus(&self);
}
struct LabelQT {}
impl Label for LabelQT {
    fn onfocus(&self) {
        println!("Onfocus QT label");
    }
}
struct LabelGTK {}
impl Label for LabelGTK {
    fn onfocus(&self) {
        println!("Onfocus GTK label");
    }
}
trait WidgetFactory {
    // По сути фабричные методы из паттерна "фабричный метод"
    fn create_button(&self) -> Box<dyn Button>;
    fn create_label(&self) -> Box<dyn Label>;
}
struct QtWidgetFactory {}
impl WidgetFactory for QtWidgetFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(ButtonQT {})
    }
    fn create_label(&self) -> Box<dyn Label> {
        Box::new(LabelQT {})
    }
}
struct GtkWidgetFactory {}
impl WidgetFactory for GtkWidgetFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(ButtonGTK {})
    }
    fn create_label(&self) -> Box<dyn Label> {
        Box::new(LabelGTK {})
    }
}
fn main() {
    // Мы выбираем UI GTK и все элементы создаются в этом стиле
    let factory = GtkWidgetFactory {};
    let button = factory.create_button();
    let label = factory.create_label();
    button.click();
    label.onfocus();
}
