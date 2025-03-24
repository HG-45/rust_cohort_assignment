struct Student{
    name: String,
    score:u32,
}
fn main () {
/*the names and scores */
let students = vec![
    Student {name:String::from("Thor"),score:66},
    Student {name:String::from("Wonder"),score:76},
    Student {name:String::from("Bat"),score:45},
    Student {name:String::from("Girl"),score:68},
    Student {name:String::from("Kent"),score:98},
    Student {name:String::from("Tony"),score:76},
    Student {name:String::from("Barney"),score:70},
    Student {name:String::from("Joker"),score:39},
    Student {name:String::from("Parker"),score:34},
    Student {name:String::from("Queen"),score:90},
];
let total_score:u32 = students.iter().map(|s|s.score).sum();
let average_score= total_score as f32/ students.len()as f32;

let mut highest = &students[0];
for student in &students {
    if student.score> highest.score{
        highest=student;
    }
}

let mut passed = 0;
let mut failed= 0;

for student in &students{
    if student.score >= 40{
        passed += 1;
    }
    else {
        failed +=1; 
    }
}

println! {"Total score of all student: \n {}",total_score};
println! {"Average score:\n  {:.2}", average_score};
println! {"The highest scoring student and his score: \n {},{}",highest.name,highest.score};
println! {"total number of students that passed:\n{}",passed};
println! {"Total number of students that failed:\n {} ",failed};
}
