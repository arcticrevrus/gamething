use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Serialize)]
pub enum ObjectType {
    Wall,
    Enemy(EnemyType),
    Player,
    Item(ItemType),
    Water,
}

#[derive(Clone, Serialize)]
pub enum EnemyType {
    Octorok,
    Dodongo,
}

#[derive(Clone, Serialize)]
pub enum ItemType {
    Rupee,
    Heart,
}

#[derive(Clone, PartialEq, Serialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Serialize)]
pub struct Objects {
    pub object_type: Option<ObjectType>,
    pub position: Position,
    pub facing: Option<Direction>,
    pub pathable: bool,
    pub usable: bool,
    pub max_health: Option<u8>,
    pub current_health: Option<u8>,
    pub damage: Option<u8>,
    pub representation: Vec<char>,
}

impl Objects {
    fn move_object(&mut self, direction: Direction, playfield: PlayField) {
        match direction {
            Direction::Up if check_collision(self, &direction, &playfield) => self.position.x += 1,
            Direction::Down if check_collision(self, &direction, &playfield) => {
                self.position.y -= 1
            }
            Direction::Left if check_collision(self, &direction, &playfield) => {
                self.position.x -= 1
            }
            Direction::Right if check_collision(self, &direction, &playfield) => {
                self.position.x += 1
            }
            _ => (),
        };
    }

    fn take_damage(&mut self, damage: u8) {
        if self.current_health.is_some() {
            if self.current_health < Some(damage) {
                self.current_health = Some(0);
            } else {
                self.current_health = Some(self.current_health.unwrap() - damage);
            }
        }
    }

    fn heal_damage(&mut self, healing: u8) {
        if self.current_health.is_some() {
            if self.max_health > { Some(self.current_health.unwrap() + healing) } {
                self.current_health = self.max_health;
            } else {
                self.current_health = Some(self.current_health.unwrap() + healing);
            }
        }
    }
}

#[derive(Clone, PartialEq, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct PlayField {
    pub entities: Vec<Objects>,
}

#[derive(Serialize)]
pub struct MapFormat {
    pub objects: Vec<Objects>,
}

impl MapFormat {
    pub fn write_to_disk(&self) {
        let mut file = File::create_new("map.map").unwrap_or_else(|error| {
            panic!("Problem creating file: {error:?}");
        });

        let json_data = serde_json::to_string(&self).expect("Failed to serialize map");
        file.write_all(json_data.as_bytes())
            .expect("Failed to write map data to file");
        println!("Wrote map to disk");
    }
}

pub fn check_collision(object: &Objects, direction: &Direction, playfield: &PlayField) -> bool {
    match direction {
        Direction::Up => {
            if object.position.y == 0 {
                return true;
            }

            let mut collides = false;
            for entity in playfield.entities.clone() {
                if entity.position.y == object.position.y - 1 {
                    collides = true;
                }
            }
            collides
        }
        Direction::Down => {
            if object.position.y == crate::MAX_HEIGHT {
                return true;
            }

            let mut collides = false;
            for entity in playfield.entities.clone() {
                if entity.position.y == object.position.y + 1 {
                    collides = true;
                }
            }
            collides
        }
        Direction::Left => {
            if object.position.x == 0 {
                return true;
            }

            let mut collides = false;
            for entity in playfield.entities.clone() {
                if entity.position.x == object.position.x - 1 {
                    collides = true;
                }
            }
            collides
        }
        Direction::Right => {
            if object.position.x == crate::MAX_WIDTH {
                return true;
            }

            let mut collides = false;
            for entity in playfield.entities.clone() {
                if entity.position.x == object.position.x - 1 {
                    collides = true;
                }
            }
            collides
        }
    }
}
