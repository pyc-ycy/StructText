fn main() {
    /*let mut s1 = Student {
        name: String::from("张三"),
        age: 23,
        grade: String::from("高三"),
        state: true,
    };
    println!("学生{}，年纪{}，年级{}，在学状态{}", s1.name, s1.age, s1.grade, s1.state);
    s1.grade = String::from("大二");
    println!("学生{}，年纪{}，年级{}，在学状态{}", s1.name, s1.age, s1.grade, s1.state);*/
    let s2 = build_student(String::from("御承扬"), 21, String::from("大三"), true);
    let s3 = build_student2(String::from("风无痕"), 24, String::from("毕业"), false);
    println!("学生{}，年纪{}，年级{}，在学状态{}", s2.name, s2.age, s2.grade, s2.state);
    println!("学生{}，年纪{}，年级{}，在学状态{}", s3.name, s3.age, s3.grade, s3.state);
}

struct Student {
    name: String,
    age: u32,
    grade: String,
    state: bool,
}

fn build_student(_name: String, _age: u32, _grade: String, _state: bool) -> Student {
    Student {
        name: _name,
        age: _age,
        grade: _grade,
        state: _state,
    }
}

fn build_student2(name:String, age:u32, grade:String, state:bool) -> Student {
    Student{
        name,
        age,
        grade,
        state
    }
}