/**
 * Это способ делегирования логики создания объектов (instantiation logic)
 * дочерним классам.
 *
 * В классо-ориентированном программировании (class-based programming)
 * фабричным методом называют порождающий шаблон проектирования, использующий
 * генерирующие методы (factory method) для решения проблемы создания объектов
 * без указания для них конкретных классов. Объекты создаются посредством вызова
 * не конструктора, а генерирующего метода, определённого в интерфейсе и
 * реализованного дочерними классами либо реализованного в базовом классе и,
 * опционально, переопределённого (overridden) производными классами
 * (derived classes).
 *
 * Когда использовать:
 * - Этот шаблон полезен для каких-то общих обработок в классе, но требуемые
 *   подклассы динамически определяются в ходе выполнения (runtime). То есть
 *   когда клиент не знает, какой именно подкласс может ему понадобиться.
*/

trait Interviewer {
    fn ask(&self);
}

struct Developer {}
impl Interviewer for Developer {
    fn ask(&self) {
        println!("Ask developer");
    }
}
struct Tester {}
impl Interviewer for Tester {
    fn ask(&self) {
        println!("Ask tester");
    }
}

trait Interview {
    // Фабричный метод
    fn make_interviewer(&self) -> Box<dyn Interviewer>;
}
struct InterviewWithDeveloper {}
impl Interview for InterviewWithDeveloper {
    fn make_interviewer(&self) -> Box<dyn Interviewer> {
        Box::new(Developer {})
    }
}
struct InterviewWithTester {}
impl Interview for InterviewWithTester {
    fn make_interviewer(&self) -> Box<dyn Interviewer> {
        Box::new(Tester {})
    }
}
fn main() {
    let interview = InterviewWithDeveloper {};
    let interviewer = interview.make_interviewer();
    interviewer.ask();
}
