use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Name {
    name: String,
}

pub fn reverse_name(name: &Name) -> Name {
    let reversed_name = name.name.chars().rev().collect::<String>();
    Name {
        name: reversed_name,
    }
}

pub fn store_name(name: &Name) -> Result<(), Box<dyn std::error::Error>> {
    // Implement your database storing logic here
    Ok(())
}

pub async fn handle_request(req: tide::Request<()>) -> tide::Result {
    let name: Name = req.body_json().await?;
    store_name(&name)?;
    let reversed_name = reverse_name(&name);
    Ok(tide::Response::new(200).body_json(&reversed_name)?)
}