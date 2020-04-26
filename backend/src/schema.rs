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
#[graphql(description = "A human being in the Rebuild of Evangelion")]
struct Human {
    id: ID,
    name: String,
    appears_in: Vec<Episode>,
}

#[derive(GraphQLObject)]
#[graphql(description = "An angl creature in the Rebuild of Evangelion")]
struct Angel {
    id: ID,
    name: String,
    appears_in: Vec<Episode>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A cyborg in the Rebuild of Evangelion")]
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

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    fn createHuman(id: ID) -> FieldResult<Human> {
        Ok(Human {
            id: ID::from("1".to_owned()),
            name: "Shinji Ikari".to_owned(),
            appears_in: vec![Episode::Jo, Episode::Ha, Episode::Q],
        })
    }
}
