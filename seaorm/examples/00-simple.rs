use sea_orm::{entity::*, error::*, prelude::*, Database, DbBackend, Schema};
use sea_query::table::TableCreateStatement;

#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}


#[async_std::main]
async fn main() -> Result<(), DbErr> {
    let db = Database::connect("sqlite::memory:").await?;
    let schema = Schema::new(DbBackend::Sqlite);
    //let stmt: TableCreateStatement = schema.create_table_from_entity(Model);

    // let result = db.execute(db.get_database_backend().build(&stmt)).await;

    let arsenal = ActiveModel {
        name: Set(String::from("Arsenal")),
        ..Default::default()
    };
    let result = arsenal.insert(&db).await?;

    println!("Inserted {} with ID {}", result.name, result.id);

    Ok(())
}
