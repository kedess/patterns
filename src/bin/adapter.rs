/**
 * Шаблон «Адаптер» позволяет помещать несовместимый объект в обёртку, чтобы он
 * оказался совместимым с другим классом.
 * Шаблон проектирования «Адаптер» позволяет использовать интерфейс
 * существующего класса как другой интерфейс. Этот шаблон часто применяется для
 * обеспечения работы одних классов с другими без изменения их исходного кода.
**/

trait Document {
    fn print(&self);
}
struct Text {
    content: String,
}
impl Text {
    fn new(content: &str) -> Self {
        Text {
            content: content.to_string(),
        }
    }
}
impl Document for Text {
    fn print(&self) {
        println!("{}", self.content);
    }
}
struct Image {
    description: String,
    #[allow(unused)]
    data: Vec<u8>,
}
impl Image {
    fn new(description: &str, data: &[u8]) -> Self {
        Image {
            description: description.to_string(),
            data: Vec::from(data),
        }
    }
    fn show(&self) {
        println!("{}", self.description);
    }
}
struct ImageAdapter {
    image: Image,
}
impl ImageAdapter {
    fn new(image: Image) -> Self {
        ImageAdapter { image }
    }
}
impl Document for ImageAdapter {
    fn print(&self) {
        self.image.show();
    }
}
fn main() {
    let documents: Vec<Box<dyn Document>> = vec![Box::new(
        Text::new("text")),
        Box::new(ImageAdapter::new(Image::new("image", &[]))),
    ];
    for doc in documents{
        doc.print();
    }
}
