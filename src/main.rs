enum Kind {
    Player,
    Enemy,
}

struct Entity {
    kind: Kind,
    position: (i32, i32),
}

fn main() {
    let entities = vec![
        Entity {
            kind: Kind::Player,
            position: (0, 0),
        },
        Entity {
            kind: Kind::Enemy,
            position: (1, 1),
        },
        Entity {
            kind: Kind::Enemy,
            position: (2, 2),
        },
    ];

    let enemies = entities.iter().filter(|e| matches!(e.kind, Kind::Enemy));
    let players = entities.iter().filter(|e| matches!(e.kind, Kind::Player));

    let _closest_enemy = closest_entity(enemies, (0, 0));
    let _closest_player = closest_entity(players, (0, 0));
}

fn distance_to_entity(entity: &Entity, position: (i32, i32)) -> i32 {
    (entity.position.0 - position.0).abs() + (entity.position.1 - position.1).abs()
}

fn closest_entity<'a>(
    entities: impl Iterator<Item = &'a Entity>,
    position: (i32, i32),
) -> Option<&'a Entity> {
    entities.min_by_key(|e| distance_to_entity(e, position))
}
