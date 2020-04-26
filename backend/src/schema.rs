use juniper::FieldResult;

use juniper::ID;

enum Episode {
    Jo,
    Ha,
    Q,
}

struct Human {
    id: ID,
    name: String,
    appears_in: Vec<Episode>,
}

struct Angel {
    id: ID,
    name: String,
    appears_in: Vec<Episode>,
}

struct Evangelion {
    id: ID,
    name: String,
    appears_in: Vec<Episode>,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn human(id: String) -> FieldResult<Human> {
        Ok(Human {
            id: ID::from("1".to_owned()),
            name: "Shinji Ikari".to_owned(),
            appears_in: vec![Episode::Jo, Episode::Ha, Episode::Q],
        })
    }
}
