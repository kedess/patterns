/**
 * Легковес — это структурный паттерн проектирования, который позволяет вместить
 * бóльшее количество объектов в отведённую оперативную память. Легковес экономит память,
 * разделяя общее состояние объектов между собой, вместо хранения одинаковых данных
 * в каждом объекте.
 *
 * Когда применяется:
 * - Когда не хватает оперативной памяти для поддержки всех нужных объектов
**/
/*
* TreeExtendData занимает много памяти, будем кешировать это поле
 */
use std::{collections::HashMap, rc::Rc};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    Red,
    Green,
    Yellow,
}
#[allow(unused)]
struct TreeExtendData {
    color: Color,
    texture_name: String,
    texture: Vec<u8>,
}
#[allow(unused)]
struct Tree {
    x: f32,
    y: f32,
    data: Rc<TreeExtendData>,
}
struct Forest {
    trees: Vec<Tree>,
}
impl Forest {
    fn new() -> Self {
        Forest { trees: Vec::new() }
    }
    fn add(&mut self, tree: Tree) {
        self.trees.push(tree);
    }
}

struct TreeFactory {
    cache: HashMap<(Color, String), Rc<TreeExtendData>>,
}

impl TreeFactory {
    fn new() -> Self {
        TreeFactory {
            cache: Default::default(),
        }
    }
    fn make(
        &mut self,
        color: Color,
        texture_name: &str,
        texture: &[u8],
        x: f32,
        y: f32,
    ) -> Tree {
        let key = (color, texture_name.to_string());
        if !self.cache.contains_key(&key) {
            let data = TreeExtendData {
                color,
                texture_name: texture_name.to_string(),
                texture: texture.to_vec(),
            };
            self.cache.insert(key.clone(), Rc::new(data));
        }
        Tree {
            x,
            y,
            data: self.cache.get(&key).unwrap().clone(),
        }
    }
}
fn main() {
    let mut forest = Forest::new();
    let mut factory = TreeFactory::new();
    let tree1 = factory.make(Color::Red, "a", &vec![0; 1024], 10.0, 10.0);
    let tree2 = factory.make(Color::Green, "b", &vec![0; 1024], 20.0, 20.0);
    let tree3 = factory.make(Color::Yellow, "c", &vec![0; 1024], 30.0, 30.0);
    let tree4 = factory.make(Color::Red, "a", &vec![0; 1024], -10.0, -10.0);
    let tree5 = factory.make(Color::Green, "b", &vec![0; 1024], -20.0, -20.0);
    let tree6 = factory.make(Color::Yellow, "c", &vec![0; 1024], -30.0, -30.0);
    forest.add(tree1);
    forest.add(tree2);
    forest.add(tree3);
    forest.add(tree4);
    forest.add(tree5);
    forest.add(tree6);
}
