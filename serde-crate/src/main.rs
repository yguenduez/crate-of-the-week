use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Hans {
    is_boss: bool,
    company: String,
}

fn main() {
    let hans = Hans {
        is_boss: true,
        company: String::from("EHDS"),
    };

    let json = serde_json::to_string(&hans).unwrap();

    println!("{}", json);

    let stringy = r#"{"is_boss":true,"company":"EHDS"}"#;
    let other_hans: Hans = serde_json::from_str(stringy).unwrap();

    println!("{:?}", other_hans);
}
