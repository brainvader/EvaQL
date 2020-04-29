use juniper::ID;
use juniper::{FieldResult, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLEnum, Clone)]
pub enum Episode {
    Jo,
    Ha,
    Q,
}

#[derive(GraphQLObject, Clone)]
#[graphql(description = "A human being in the Rebuild of Evangelion")]
pub struct Human {
    pub id: ID,
    pub name: String,
    pub appears_in: Vec<Episode>,
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

#[derive(Clone)]
pub struct EvaContext {
    pub human: Human,
}

impl juniper::Context for EvaContext {}

pub struct QueryRoot;

#[juniper::object(Context = EvaContext)]
impl QueryRoot {
    fn human(context: &EvaContext) -> FieldResult<Human> {
        let Human {
            id,
            name,
            appears_in,
        } = &context.human;
        Ok(Human {
            id: id.to_owned(),
            name: name.to_string(),
            appears_in: appears_in.to_vec(),
        })
        // Ok(Human {
        //     id: ID::from("1".to_owned()),
        //     name: "Shinji Ikari".to_owned(),
        //     appears_in: vec![Episode::Jo, Episode::Ha, Episode::Q],
        // })
    }
}

pub struct MutationRoot;

#[juniper::object(Context = EvaContext)]
impl MutationRoot {
    fn createHuman(id: ID) -> FieldResult<Human> {
        Ok(Human {
            id: ID::from("1".to_owned()),
            name: "Shinji Ikari".to_owned(),
            appears_in: vec![Episode::Jo, Episode::Ha, Episode::Q],
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
