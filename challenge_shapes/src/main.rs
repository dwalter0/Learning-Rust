struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        return self.width * self.height;
    }

    fn scale(&mut self,scale_by: f64) {
        self.width = self.width * scale_by;
        self.height = self.height * scale_by;
    }

    fn new(height: f64, width: f64) -> Rectangle {
        Rectangle {
            height: height,
            width: width
        }
    }
}

fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}