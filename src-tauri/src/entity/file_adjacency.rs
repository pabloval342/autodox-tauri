//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "file_adjacency")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub tree_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub parent_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub child_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::file_node::Entity",
        from = "Column::ChildId",
        to = "super::file_node::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    FileNode2,
    #[sea_orm(
        belongs_to = "super::file_node::Entity",
        from = "Column::ParentId",
        to = "super::file_node::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    FileNode1,
    #[sea_orm(
        belongs_to = "super::file_tree::Entity",
        from = "Column::TreeId",
        to = "super::file_tree::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    FileTree,
}

impl Related<super::file_tree::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileTree.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
