// Тестовый файл для проверки компиляции
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct TestHolder {
    data: Arc<Mutex<Vec<i32>>>,
}

impl TestHolder {
    pub fn new() -> Self {
        TestHolder {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add(&self, value: i32) {
        let mut data = self.data.lock().await;
        data.push(value);
    }

    pub async fn len(&self) -> usize {
        let data = self.data.lock().await;
        data.len()
    }
}

#[tokio::main]
async fn main() {
    let holder = TestHolder::new();
    holder.add(42).await;
    println!("Length: {}", holder.len().await);
}
