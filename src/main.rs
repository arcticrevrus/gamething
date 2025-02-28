mod objects;
mod render;

const MAX_HEIGHT: i32 = 24;
const MAX_WIDTH: i32 = 80;

fn main() {
    let mut objects: objects::PlayField = objects::PlayField {
        entities: Vec::new(),
    };

    for i in 1..MAX_HEIGHT {
        for j in 1..MAX_WIDTH {
            if i <= { MAX_HEIGHT / 8 }
                || i >= { MAX_HEIGHT - { MAX_HEIGHT / 8 } }
                || j <= { MAX_WIDTH / 8 }
                || j >= { MAX_WIDTH - { MAX_WIDTH / 8 } }
            {
                objects.entities.push(objects::Objects {
                    object_type: Some(objects::ObjectType::Wall),
                    position: objects::Position { x: j, y: i },
                    facing: None,
                    pathable: false,
                    usable: false,
                    max_health: None,
                    current_health: None,
                    damage: None,
                    representation: vec!['^', 'v', '<', '>'],
                })
            }
        }
    }

    objects.entities.push(objects::Objects {
        object_type: Some(objects::ObjectType::Player),
        position: objects::Position {
            x: { MAX_WIDTH / 2 },
            y: { MAX_HEIGHT / 2 },
        },
        facing: Some(objects::Direction::Down),
        pathable: true,
        usable: false,
        max_health: Some(3 * 4),
        current_health: Some(3 * 4),
        damage: Some(1),
        representation: vec!['X'],
    });
    render::render(objects);
}
