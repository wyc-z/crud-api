use sea_orm_migration::{async_trait::async_trait, prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("categories")
                    .if_not_exists()
                    .col(unsigned_uniq("id").primary_key().auto_increment())
                    .col(string_len_uniq("name", 20))
                    .col(string_len_null("description", 60))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("brands")
                    .if_not_exists()
                    .col(unsigned_uniq("id").primary_key().auto_increment())
                    .col(string_len_uniq("name", 20))
                    .col(string_len_null("description", 60))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("products")
                    .if_not_exists()
                    .col(unsigned_uniq("id").primary_key().auto_increment())
                    .col(string_len("name", 50).not_null())
                    .col(unsigned_null("brand_id"))
                    .col(unsigned("category_id").not_null())
                    .col(decimal_len("cost", 10, 2).not_null())
                    .col(decimal_len("price", 10, 2).not_null())
                    .col(unsigned("stock").not_null())
                    .col(string_len_uniq("sku", 50).not_null())
                    .col(string_len_null("description", 120))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_products_brand")
                            .from("products", "brand_id")
                            .to("brands", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_products_categories")
                            .from("products", "category_id")
                            .to("categories", "id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("products").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("categories").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("brands").to_owned())
            .await
    }
}
