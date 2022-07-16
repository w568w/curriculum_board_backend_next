//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "course_review")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub course_id: i32,
    #[sea_orm(primary_key)]
    pub review_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::course::Entity",
        from = "Column::CourseId",
        to = "super::course::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Course,
    #[sea_orm(
        belongs_to = "super::review::Entity",
        from = "Column::ReviewId",
        to = "super::review::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Review,
}


impl ActiveModelBehavior for ActiveModel {}
