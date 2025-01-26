fn main() {
    let s1 = Student{
        grades: vec![10, 10],
    };

    let s2 = Student{
        grades: vec![10, 10],
    };

    let s1_avg_grade = s1.process_grades();
    match s1_avg_grade {
        Err(e) => println!("{e}"),
        Ok(Some(value)) => println!("student avg grade: {value}"),
        _ => {}
    }

    let c1 = Classroom{
        students: vec![s1, s2]
    };

    let c1_avg_grade = c1.process_grades();
    match c1_avg_grade {
        Err(e) => println!("{e}"),
        Ok(Some(value)) => println!("class room avg grade: {value}"),
        _ => {}
    }

}

trait GradeProcessor{
    fn process_grades(&self) -> Result<Option<f64>, String>;
}

struct Student{
    grades: Vec<i32>,
}

impl GradeProcessor for Student{
    fn process_grades(&self)-> Result<Option<f64>, String>{
        let grades = &self.grades;
        match  grades.is_empty(){
            true => Err(format!("grade not found")),
            false => {
                let sum:i32 = grades.iter().sum();
                let len = grades.len();
                Ok(Some(sum as f64/len as f64))
            }
        }
    }
}

struct Classroom{
    students: Vec<Student>,
}

impl GradeProcessor for Classroom{
    fn process_grades(&self)-> Result<Option<f64>, String>{
        if self.students.is_empty(){
            return Err(format!("no students in classroom"));
        }else{
            let mut sums =0.0;
            for s in &self.students{
                let sum = s.process_grades();
                match sum{
                    Ok(Some(value)) => sums+=value,
                    Ok(None) => println!("no value found"),
                    Err(e) => {
                        println!("{e}")
                    },
                }
            }
            if sums==0.0{
                return Err(format!("no valid grade found"));
            }
            Ok(Some(sums/self.students.len() as f64))
        }
    }
}