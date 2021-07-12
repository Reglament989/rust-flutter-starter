use serde::{Deserialize};


#[derive(Deserialize, Debug)]
pub struct Todo {
    #[serde(rename(deserialize = "userId"))]
    pub user_id: i8,
    pub id: i8,
    pub title: String,
    pub completed: bool
}

impl PartialEq for Todo
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

const URL: &str = "https://jsonplaceholder.typicode.com/todos/1";

pub fn get_one_todo() -> Result<Todo, Box<dyn std::error::Error>> {
    let resp: Todo = reqwest::blocking::get(URL)?
        .json()?;
    println!("{:#?}", resp);
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_todo() {
        let ok = get_one_todo();
        let example_todo = Todo {
            user_id: 1,
            id: 1,
            title: String::from("delectus aut autem"),
            completed: false
        };
        assert_eq!(ok.unwrap(), example_todo)
    }
}