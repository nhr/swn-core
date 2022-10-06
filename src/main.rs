use rand::{thread_rng, Rng};

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

  // The first planet in a system is always the primary
  primary: bool,

  // Additional planets (if any) have some connection to the primary.
  // (See 'Adding Other Worlds' under 'Additional System Points of Interest')
  origin: String,
  relationship: String,
  contact_point: String,
}

struct PointOfInterest {
  poi_type: String,
  name: String,
  occupied_by: String,
  situation: String,
}

struct StarSystem {
  name: String,
  sector_position: String,
  planets: Vec<Planet>,
  pois: Vec<PointOfInterest>
}

struct AlienRace {
  name: String,
  body_type: String,
  lens: String,
  social_structure: String,
}

struct Faction {
  name: String,
  hit_points: u8,
  force: u8,
  cunning: u8,
  wealth: u8,
  fac_creds: u64,
  xp: u64,
  homeworld: String,
  tags: Vec<FactionTag>,
  assets: Vec<FactionAsset>,
  goal: FactionGoal,
}

struct FactionTag {
  title: String,
  description: String,
  effect: String,
}

struct FactionAsset {
  asset_type: String,
  name: String,
  description: String,
  hit_points: u8,
  purchase_cost: u64,
  maintenance_cost: u64,
  attack: u8,
  counterattack: u8,
  location: String,
  tech_level: u8,
}

struct FactionGoal {
  name: String,
  description: String,
}

struct NPC {
  name: String,
  initial_manner: String,
  default_deal_outcome: String,
  motivation: String,
  want: String,
  power: String,
  hook: String,
}

struct Sector {
  name: String,
  star_systems: Vec<StarSystem>,
  factions: Vec<Faction>,
  aliens: Vec<AlienRace>,
  npcs: Vec<NPC>,
}

fn roll(count: u8, sides: u8, modifier: u8) -> u8 {
  let mut rng = thread_rng();
  let mut total: u8 = 0;
  for dice_roll in 0..count {
    let x: u8 = rng.gen_range(1..(sides + 1));
    total = total + x
  }
  total + modifier
}

fn generate_sector_name() -> String {
  "".to_string()
}

fn generate_star_system_name() -> String {
  "".to_string()
}

fn generate_planet_name() -> String {
  "".to_string()
}

fn generate_sector() -> Sector {
  Sector {
    name: generate_sector_name(),
    star_systems: (0..roll(1,10,20)).map(|_| generate_star_system()).collect(),
    factions: (0..roll(1,5,7)).map(|_| generate_faction()).collect(),
    aliens: (0..roll(1,3,5)).map(|_| generate_alien()).collect(),
    npcs: (0..100).map(|_| generate_npc()).collect(),
  }
}

fn generate_star_system() -> StarSystem {
  StarSystem {
    name: generate_star_system_name(),
    sector_position: "B1".to_string(),
    planets: (0..1).map(|_| generate_planet(true)).collect(),
    pois: (0..1).map(|_| generate_poi()).collect(),
  }
}

fn generate_planet(is_primary: bool) -> Planet {
  Planet {
    name: generate_planet_name(),
    world_tags: (0..2).map(|_| generate_world_tag()).collect(),
    atmosphere: random_lookup("atmosphere".to_string()),
    temperature: random_lookup("temperature".to_string()),
    biosphere: random_lookup("biosphere".to_string()),
    population: random_lookup("population".to_string()),
    tech_level: random_lookup("tech_level".to_string()),
    primary: is_primary,
    origin: if is_primary { "".to_string() } else { random_lookup("origin".to_string()) },
    relationship: if is_primary { "".to_string() } else { random_lookup("relationship".to_string()) },
    contact_point: if is_primary { "".to_string() } else { random_lookup("contact_point".to_string()) },
  }
}

fn generate_world_tag() -> WorldTag {

}

fn generate_poi() -> PointOfInterest {

}

fn generate_alien() -> AlienRace {

}

fn generate_faction() -> Faction {

}

fn generate_npc() -> NPC {

}

fn random_lookup(lookup_type: String) -> String {
  let result = "";
  if lookup_type == "atmosphere" {
    result = "ATMOSPHERE";
  } else if lookup_type == "temperature" {
    result = "TEMPERATURE";
  } else if lookup_type == "biosphere" {
    result = "BIOSPHERE";
  } else if lookup_type == "population" {
    result = "POPULATION";
  } else if lookup_type == "tech_level" {
    result = "TECH LEVEL";
  } else if lookup_type == "origin" {
    result = "ORIGIN";
  } else if lookup_type == "relationship" {
    result = "RELATIONSHIP";
  } else if lookup_type == "contact_point" {
    result = "CONTACT POINT";
  }
  result.to_string()
}

fn main() {
    println!("Hello, world number {}!", roll(1,8,0));
}
