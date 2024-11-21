// Classic struct (Like C#, Java, C++ && C)
struct Student {
    name: String,
    level: u8,
    remote: bool
}

// Tuple struct
struct Grades(char, char, char, char, f32);

fn main() {
    let student_1 = Student {
        name: String::from("John"),
        level: 3,
        remote: true
    };

    let student_2 = Student {
        name: String::from("Franco"),
        level: 1,
        remote: false
    };

    let _mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let _mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!("Name: {}, Level: {}, Remote: {}, Grades: {} {} {} {}, Average: {}", student_1.name, student_1.level, student_1.remote, _mark_1.0, _mark_1.1, _mark_1.2, _mark_1.3, _mark_1.4);
    println!("Name: {}, Level: {}, Remote: {}, Grades: {} {} {} {}, Average: {}", student_1.name, student_1.level, student_1.remote, _mark_1.0, _mark_1.1, _mark_1.2, _mark_1.3, _mark_1.4);
}
