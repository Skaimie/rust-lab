#[derive(Debug)]
struct Player {
    name: String,
    health: i32,
    level: u32,
}

impl Player {
    // Associated function
    fn new(name: String) -> Player {
        Player {
            name,
            health: 100,
            level: 1,
        }
    }

    // Immutable method
    fn show_status(&self) {
        println!(
            "{} | HP: {} | Level: {}",
            self.name, self.health, self.level
        );
    }

    // Mutable method
    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    // Mutable method
    fn level_up(&mut self) {
        self.level += 1;
        self.health = 100;
    }
}

fn main() {
    let mut player = Player::new(String::from("Skaimie"));

    player.show_status();

    player.take_damage(30);

    player.show_status();

    player.level_up();

    player.show_status();
    println!("{:?}", player.name);
    println!("{:#?}", player);
}
