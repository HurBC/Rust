use core::fmt;

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait AsJson {
    fn as_json(&self) -> String;
}

struct Person {
    name: String,
    age: u8,
    favorite_fruit: String
}

struct Dog {
    name: String,
    color: String,
    likes_petting: bool
}

impl AsJson for Person {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
            self.name, self.age, self.favorite_fruit
        )
    }
}

impl AsJson for Dog {
    fn as_json(&self) -> String {
        format!(
	        r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
	        self.name, self.color, self.likes_petting
	    )
    }
}

#[derive(Debug)]
struct Counter {
    count: usize,
    length: usize
}

impl Counter {
    fn new(length: usize) -> Counter {
        Counter {
            count: 0,
            length
        }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

fn send_data_as_json(value: &impl AsJson) {
    println!("Sending JSON data to server...");
    println!("-> {}", value.as_json());
    println!("Done!\n");
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    if p1 == p2 {
        println!("Are Equals");
    } else {
        println!("They are not equals");
    }

    println!("{}", p1);
    println!("{:?}", p1);

    let franco = Person {
        name: "Franco".to_string(),
        age: 19,
        favorite_fruit: "Frutilla".to_string()
    };
    // Maraka
    let vania = Dog {
        name: "Vania".to_string(),
        color: "Black".to_string(),
        likes_petting: true
    };
    
    send_data_as_json(&franco);
    send_data_as_json(&vania);

    let mut counter = Counter::new(6);

    println!("Counter just created: {:#?}", counter);

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), Some(6));
    assert_eq!(counter.next(), None);

    println!("Counter just finished: {:#?}", counter);

    for number in Counter::new(10) {
        println!("{}", number);
    }

    let sum_until_10: usize = Counter::new(10).sum();

    assert_eq!(sum_until_10, 55);
    println!("Sum until 10: {}", sum_until_10);

    let powers_of_2: Vec<usize> = Counter::new(8).map(
        |n| 2usize.pow(n as u32)
    ).collect();

    assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);

    println!("Powers of 2: {:#?}", powers_of_2);

    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(Container::new(String::from("Bar")).value, String::from("Bar"));
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));
}

// Exercise
struct Container<T> {
    value: T
}

impl<T> Container<T> {
    pub fn new(value: T) -> Container<T> {
        Container {
            value
        }
    }
}

// Exercise 2
struct Groups<T> {
    inner: Vec<T>
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}

impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }

        let mut cursor = 0;

        let first = &self.inner[0];

        for element in &self.inner[1..] {
            if element == first {
                cursor += 1;
            } else {
                break;
            }
        }

        let items = self.inner.drain(0..cursor).collect();

        Some(items)
    }
}
