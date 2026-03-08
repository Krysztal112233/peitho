use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::sea_orm::DbBackend;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(HistoryRole::Type)
                    .values([
                        HistoryRole::User,
                        HistoryRole::Assistant,
                        HistoryRole::System,
                        HistoryRole::Tool,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Histories::Table)
                    .if_not_exists()
                    .col(pk_auto(Histories::Id))
                    .col(string(Histories::SessionId).not_null())
                    .col(text(Histories::Content).not_null())
                    .col(
                        ColumnDef::new(Histories::Role)
                            .enumeration(
                                HistoryRole::Type,
                                [
                                    HistoryRole::User,
                                    HistoryRole::Assistant,
                                    HistoryRole::System,
                                    HistoryRole::Tool,
                                ],
                            )
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(Histories::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_histories_session_created_at")
                    .table(Histories::Table)
                    .col(Histories::SessionId)
                    .col(Histories::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() != DbBackend::Postgres {
            return Err(DbErr::Migration(
                "this migration only supports PostgreSQL".to_owned(),
            ));
        }

        manager
            .drop_table(Table::drop().table(Histories::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(HistoryRole::Type).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Histories {
    Table,

    Id,
    SessionId,
    Content,
    Role,
    CreatedAt,
}

#[derive(DeriveIden)]
enum HistoryRole {
    #[sea_orm(iden = "history_role")]
    Type,
    User,
    Assistant,
    System,
    Tool,
}
