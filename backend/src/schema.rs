use juniper::ID;

enum Episode {
    Jo,
    Ha,
    Q,
}

struct Human {
    id: ID,
    name: String,
    appears_in: Vec<Episode>
}

struct Angel {
    id: ID,
    name: String
    appears_in: Vec<Episode>
}

struct Evangelion {
    id: ID,
    name: String,
    appears_in: Vec<Episode>
}