/*
* Шаблон «Посетитель» позволяет добавлять будущие операции для объектов без их модифицирования.
 */

 trait Visitor {
    fn visit_plant(&self);
    fn visit_shop(&self);
    fn visit_warehouse(&self);
 }

 struct FirstVisitor{}
 impl Visitor for FirstVisitor {
     fn visit_plant(&self) {
         println!("First visited plant");
     }
     fn visit_shop(&self) {
        println!("First visited shop");
     }
     fn visit_warehouse(&self) {
        println!("First visited warehouse");
     }
 }
 struct SecondVisitor{}
 impl Visitor for SecondVisitor {
     fn visit_plant(&self) {
         println!("Second visited plant");
     }
     fn visit_shop(&self) {
        println!("Second visited shop");
     }
     fn visit_warehouse(&self) {
        println!("Second visited warehouse");
     }
 }
 trait Building {
    fn accept(&self, visitor: &dyn Visitor);    
 }

 struct Plant{}
 impl Building for Plant {
     fn accept(&self, visitor: &dyn Visitor) {
         visitor.visit_plant();
     }
 }
 struct Shop{}
 impl Building for Shop {
     fn accept(&self, visitor: &dyn Visitor) {
         visitor.visit_shop();
     }
 }
 struct Warehouse{}
 impl Building for Warehouse {
     fn accept(&self, visitor: &dyn Visitor) {
         visitor.visit_warehouse();
     }
 }
fn main() {
    let list: Vec<Box<dyn Building>> = vec![Box::new(Plant{}), Box::new(Shop{}), Box::new(Warehouse{})];
    let first_visitor = FirstVisitor{};
    for building in &list {
        building.accept(&first_visitor);
    }
    let second_visitor = SecondVisitor{};
    for building in &list {
        building.accept(&second_visitor);
    }
}