use juniper::FieldResult;
use juniper::ID;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLEnum)]
enum Episode {
    Jo,
    Ha,
    Q,
}

#[derive(GraphQLObject)]
struct Human {
    id: ID,
    name: String,
    appears_in: Vec<Episode>,
}

#[derive(GraphQLObject)]
struct Angel {
    id: ID,
    name: String,
    appears_in: Vec<Episode>,
}

#[derive(GraphQLObject)]
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
