/**
 * Позволяет объекту варьировать свое поведение в зависимости от внутреннего состояния. Извне создается впечатление, что изменился класс объекта
 * 
 * Применимость:
 * - когда поведение объекта зависит от его состояния и должно изменяться во время работы
 * - когда в коде операций встречаются состоящие из многиъ ветвей условные лператоры, в которых выбор ветви зависит от состояния
**/

trait ArticleState {
    fn edit(&mut self, article: &mut Article, text: &str);
    fn approve(&mut self, article: &mut Article);
    fn publish(&mut self, article: &mut Article);
}

struct DraftState {}
impl DraftState {
    fn new() -> Self {
        DraftState {  }
    }
}
impl ArticleState for DraftState {
    fn edit(&mut self, article: &mut Article, text: &str) {
        println!("DraftState -> DraftState");
        article.content = text.to_string();
        article.state = Some(Box::new(DraftState::new()))
    }
    fn approve(&mut self, article: &mut Article) {
        println!("DraftState -> ApprovedState");
        article.state = Some(Box::new(ApprovedState::new()))
    }
    fn publish(&mut self, article: &mut Article) {
        article.state = Some(Box::new(DraftState::new()))
    }
}

struct ApprovedState {}
impl ApprovedState {
    fn new() -> Self {
        ApprovedState {  }
    }
}
impl ArticleState for ApprovedState {
    fn edit(&mut self, article: &mut Article, text: &str) {
        println!("ApprovedState -> DraftState");
        article.content = text.to_string();
        article.state = Some(Box::new(DraftState::new()))
    }
    fn approve(&mut self, article: &mut Article) {
        article.state = Some(Box::new(ApprovedState::new()))
    }
    fn publish(&mut self, article: &mut Article) {
        println!("ApprovedState -> PublishedState");
        article.state = Some(Box::new(PublishedState::new()))
    }
}

struct PublishedState {}
impl PublishedState {
    fn new() -> Self {
        PublishedState {  }
    }
}
impl ArticleState for PublishedState {
    fn edit(&mut self, article: &mut Article, text: &str) {
        println!("PublishedState -> DraftState");
        article.content = text.to_string();
        article.state = Some(Box::new(DraftState::new()))
    }
    fn approve(&mut self, article: &mut Article) {
        article.state = Some(Box::new(PublishedState::new()))
    }
    fn publish(&mut self, article: &mut Article) {
        article.state = Some(Box::new(PublishedState::new()))
    }
}

struct Article {
    content: String,
    state: Option<Box<dyn ArticleState>>
}
impl Article {
    fn new() -> Self {
        Article { content: "".to_string(), state: Some(Box::new(DraftState::new())) }
    }
    fn set_content(&mut self, text : &str){
        if let Some(mut state) = self.state.take() {
            state.edit(self, text);
        }
    }
    fn approve(&mut self) {
        if let Some(mut state) = self.state.take() {
            state.approve(self);
        }
    }
    fn publish(&mut self) {
        if let Some(mut state) = self.state.take() {
            state.publish(self);
        }
    }
}

fn main(){
    let mut article = Article::new();
    article.set_content("Pattern state!");
    article.approve();
    article.set_content("Pattern state!!!");
    article.approve();
    article.publish();
}