use crate::objects;

pub fn render(playfield: objects::PlayField) {
    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;
    for object in &playfield.entities {}

    for y in 1..crate::MAX_HEIGHT {
        for x in 1..crate::MAX_WIDTH {
            let position = objects::Position { x, y };
            if let Some(object) = playfield
                .entities
                .iter()
                .find(|object| object.position == position)
            {
                if object.representation.len() == 1 {
                    print!("{}", object.representation.first().unwrap());
                } else if object.representation.len() == 4 {
                    match object.facing {
                        Some(objects::Direction::Up) => {
                            print!("{}", object.representation.first().unwrap())
                        }
                        Some(objects::Direction::Down) => {
                            print!("{}", object.representation.get(1).unwrap())
                        }
                        Some(objects::Direction::Left) => {
                            print!("{}", object.representation.get(2).unwrap())
                        }
                        Some(objects::Direction::Right) => {
                            print!("{}", object.representation.get(3).unwrap())
                        }
                        None => (),
                    }
                }
            } else {
                print!(" ")
            }
        }
        println!();
    }
}
