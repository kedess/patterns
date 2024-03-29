use std::collections::VecDeque;

/**
* Шаблон «Хранитель» фиксирует и хранит текущее состояние объекта, чтобы оно
* легко восстанавливалось.
*
* Шаблон «Хранитель» позволяет восстанавливать объект в его предыдущем состоянии
* (отмена через откат — undo via rollback).
*
* Когда использовать:
* - когда необходимо сохранить мгновенный снимок состояния объекта
*   (или его части), чтобы впоследствии объект можно было восстановить в том же
*   состоянии
* - когда прямое получение этого состояния раскрывает детали реализации и нарушает
*   инкапсуляцию объекта
**/

struct MementoArticle {
    content: String,
}
impl MementoArticle {
    fn new(content: &str) -> Self {
        MementoArticle {
            content: content.to_string(),
        }
    }
    fn get_content(self) -> String {
        self.content
    }
}

struct Article {
    content: String,
    memento: VecDeque<MementoArticle>,
}

impl Article {
    fn new() -> Self {
        Article {
            content: Default::default(),
            memento: Default::default(),
        }
    }
    fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }
    fn get_content(&mut self) -> &str {
        &self.content
    }
    fn save(&mut self) {
        self.memento.push_back(MementoArticle::new(&self.content));
    }
    fn restore(&mut self) {
        if let Some(memento) = self.memento.pop_back() {
            self.content = memento.get_content();
        }
    }
}

fn main() {
    let mut article = Article::new();
    article.set_content("First content");
    article.save();

    article.set_content("Second content");
    article.save();

    article.set_content("Third content");
    article.save();

    for _ in 0..3 {
        article.restore();
        println!("{}", article.get_content());
    }
}
