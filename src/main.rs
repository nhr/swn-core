
struct WorldTag {
  name: String,
  description: String,
  enemies: Vec<String>,
  friends: Vec<String>,
  complications: Vec<String>,
  things: Vec<String>,
  places: Vec<String>,
}

struct Planet {
  name: String,
  world_tags: Vec<WorldTag>,
  atmosphere: String,
  temperature: String,
  biosphere: String,
  population: String,
  tech_level: String,
}

struct StarSystem {
  planets: Vec<Planet>
}

fn main() {
    println!("Hello, world!");
}
