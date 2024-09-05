use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(User::Table).if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(Column::timestamp(User::Name).string().not_null())
                    .col(Column::string(User::Email).string().unique_key().not_null())
                    .col(Column::timestamp(User::Password).not_null())
                    .col(Column::timestamp(User::Uuid).uuid().unique_key().not_null())
                    .col(Column::timestamp(User::CreatedAt).data_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    Password,
    Uuid,
    CreatedAt,
}
