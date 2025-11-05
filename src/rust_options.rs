
struct Student{
    name: String,
    grade: Option<u32>
}

fn get_student_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32>{
     
     for student in student_db {
        if student.name == *student_name{
            return student.grade;
        }
     }
     None
}

// result in the rust standard library looks like this,
//  enum Result<T,E>{
//  OK(T),
//  Err(E)
// }

fn check_student(student_name:&String, student_db:&Vec<Student>)-> Result<(), String>{
     for students in student_db{
        if students.name == *student_name{
            return  Ok(());
        }
     }
     Err(String::from("Student not found"));
}
pub fn rust_options(){
    let student_db=vec![
        Student{
            name:String::from("James"),
            grade: Some(95),
        },
        Student{
            name:String::from("Ochula"),
            grade:Some(80)
        },
        Student{
            name:String::from("Victor"),
            grade:None
        }
    
    ];

    // let student_scores=get_student_grade(&String::from("James"), &student_db);
    
    // match  student_scores {
    //     Some(grade) => println!("Grade is : {grade}"),
    //     None => {}
    // }
    // println!("{:?}", student_scores)

    let checked_student= check_student(&String::from("James"), &student_db);
    
}