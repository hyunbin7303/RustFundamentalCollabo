#[derive(Debug)]
struct Champ {
    id : i32, 
    name : String, 
    height : u32,
    weight : u32, 
    lifestate : LifeState,
    // can_use : bool,
}

impl Default for Champ {
    fn default() -> Self {
        Self{
            id: 1,
            name: "Garen".to_string(),
            height : 100,
            weight : 100,
            lifestate: LifeState::Alive
        }
    }
}

// not using can_use flag
#[derive(Debug)]
struct ChampBuilder  {
    id : i32, 
    name : String, 
    height : u32,
    weight : u32, 
    lifestate : LifeState,
 }


#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain
}

fn use_character(champ: &Champ) {

}



impl Default for ChampBuilder {
    fn default() -> Self {
        Self {
            id: 1,
            name : "Garen".to_string(),
            height: 180,
            weight: 80,
            lifestate : LifeState::Alive,
         }
    }
}

impl ChampBuilder {
    fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self 
    }
    fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }
    fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    fn build(mut self) -> Result<Champ, String> { 
        if self.height < 300 && self.weight < 300 && !self.name.to_lowercase().contains("troll"){
            Ok(Champ {
                name : self.name, 
                id: self.id,
                height: self.height,
                weight: self.weight,
                lifestate: self.lifestate
            })
        }else {
            Err("Name must ot coontain troll..".to_string()) 
        } 
         
    }
}

fn main() {

    let champbuilder_garen = ChampBuilder::default()
                        .with_height(200)
                        .with_weight(150)
                        .with_name("KEVIN PARK troll");
                        // .build();
    println!("{:?}", champbuilder_garen);
    let champbuilder_garen = champbuilder_garen.build();
    println!("{:?}", champbuilder_garen);

    
}
