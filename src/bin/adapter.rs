/**
 * Адаптер — это структурный паттерн проектирования, который позволяет объектам
 * с несовместимыми интерфейсами работать вместе.
 * Шаблон проектирования «Адаптер» позволяет использовать интерфейс
 * существующего класса как другой интерфейс. Этот шаблон часто применяется для
 * обеспечения работы одних классов с другими без изменения их исходного кода.
 * Когда использовать:
 * - Когда вы хотите использовать сторонний класс, но его интерфейс не соответствует
 * остальному коду приложения
 * - Когда вам нужно использовать несколько существующих подклассов, но в них не хватает
 * какой-то общей функциональности, причём расширить суперкласс вы не можете
**/

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}
struct Document {
    data: String,
}
impl Document {
    fn new(data: &str) -> Self {
        Document {
            data: data.to_string(),
        }
    }
}
struct Image {
    data: Vec<u8>,
}
impl Image {
    fn new(data: &[u8]) -> Self {
        Image {
            data: data.to_vec(),
        }
    }
}
impl Serialize for Image {
    fn serialize(&self) -> Vec<u8> {
        self.data.clone()
    }
}

struct DocumentAdapter {
    document: Document,
}

impl DocumentAdapter {
    fn new(document: Document) -> Self {
        DocumentAdapter { document }
    }
}

impl Serialize for DocumentAdapter {
    fn serialize(&self) -> Vec<u8> {
        self.document.data.as_bytes().to_vec()
    }
}

fn main() {
    let objects: Vec<Box<dyn Serialize>> = vec![
        Box::new(Image::new(&[0, 0, 0, 0])),
        Box::new(DocumentAdapter::new(Document::new("document"))),
    ];
    for doc in objects {
        doc.serialize();
    }
}
