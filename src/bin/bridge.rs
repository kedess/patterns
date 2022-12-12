/**
 * Шаблон «Мост» — это предпочтение компоновки наследованию. Подробности
 * реализации передаются из одной иерархии другому объекту с отдельной
 * иерархией.
 *
 * Шаблон «Мост» означает отделение абстракции от реализации, чтобы их обе
 * можно было изменять независимо друг от друга.
**/

trait Color {
    fn fill(&self);
}

struct Black {}
impl Black {
    fn new() -> Self {
        Black {}
    }
}
impl Color for Black {
    fn fill(&self) {
        println!("Filling in black color");
    }
}

struct Red {}
impl Red {
    fn new() -> Self {
        Red {}
    }
}
impl Color for Red {
    fn fill(&self) {
        println!("Filling in red color");
    }
}

trait Shape {
    fn draw(&self);
}

struct Circle {
    color: Box<dyn Color>,
}
impl Circle {
    fn new(color: Box<dyn Color>) -> Self {
        Circle { color }
    }
}
impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing circle");
        self.color.fill();
    }
}

struct Rectangle {
    color: Box<dyn Color>,
}
impl Rectangle {
    fn new(color: Box<dyn Color>) -> Self {
        Rectangle { color }
    }
}
impl Shape for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle");
        self.color.fill();
    }
}

fn main() {
    let circle = Circle::new(Box::new(Black::new()));
    let rectangle = Rectangle::new(Box::new(Red::new()));
    circle.draw();
    rectangle.draw();
}
