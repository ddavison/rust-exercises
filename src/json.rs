use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, from_str};

#[derive(Serialize, Deserialize, Debug)]
struct Dog {
    name: String,
    year_born: u32,
    owner: DogOwner,
}

#[derive(Serialize, Deserialize, Debug)]
struct DogOwner {
    first_name: String,
}

fn main() {
    let owner = DogOwner { first_name: "Deej".to_string() };
    let teddy = Dog { name: "Teddy".to_string(), year_born: 2019, owner };
    // let tucker = Dog { name: "Tucker".to_string(), year_born: 2022, &owner };

    // serialize using to_string_pretty()
    println!("{}", to_string_pretty(&teddy).expect("Cannot serialize"));
    // println!("{}", to_string_pretty(&tucker).expect("Cannot serialize"));

    let json = r#"
        {
          "name": "Teddy",
          "year_born": 2019,
          "owner": {
            "first_name": "Deej"
          }
        }
    "#;

    // deserialize using from_value()

    let teddy: Dog = from_str(json).expect("Cannot deserialize");

    println!("{:?}", teddy);
}
