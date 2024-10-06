use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建订单表
        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Order::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Order::UserId).integer().not_null())
                    .col(ColumnDef::new(Order::ProductId).integer().not_null())
                    .col(ColumnDef::new(Order::TotalPrice).decimal().not_null())
                    .col(ColumnDef::new(Order::OrderTime).date_time().not_null())
                    .col(ColumnDef::new(Order::Status).integer().not_null())
                    .to_owned(),
            )
            .await?;
        // 创建产品表
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::Price).decimal().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除产品表
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await?;
        // 删除订单表
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    UserId,
    ProductId,
    TotalPrice,
    OrderTime,
    Status,
}
#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Name,
    Price,
}
