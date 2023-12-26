use std::time::Duration;

use tokio::time::sleep;

#[derive(Clone, Debug)]
struct Employee {
    id: u32,
    name: String,
    salary: f32,
}

impl Employee {
    fn new(id: u32, name: &str, salary: f32) -> Self {
        Self {
            id,
            name: name.to_string(),
            salary,
        }
    }
}

async fn read_details_from_db(id: u32) -> Result<Employee, String> {
    println!("query paramter: id = {id}");
    sleep(Duration::from_millis(1000)).await;

    let database = [
        Employee::new(1, "Ferris", 98_000.0),
        Employee::new(2, "Ferrris", 95_000.0),
        Employee::new(3, "Feerris", 95_000.0),
    ];

    for emp in database {
        if id == emp.id {
            println!("result: {emp:?}");
            return Ok(emp);
        }
    }

    Err("Employ record not present".to_string())
}

#[tokio::main]
async fn main() {
    let (id1, id2) = (1, 3);

    println!("Start!");
    let emp1 = read_details_from_db(id1).await.unwrap();
    let emp2 = read_details_from_db(id2).await.unwrap();
    let dif = emp1.salary - emp2.salary;

    if dif == 0.0 {
        println!("Both {} and {} earn same amount", emp1.name, emp2.name);
    } else {
        let (more, less) = if dif > 0.0 {
            (emp1, emp2)
        } else {
            (emp2, emp1)
        };

        println!(
            "{} earns ${:.1} more then {}",
            more.name,
            dif.abs(),
            less.name
        );
    }
}
