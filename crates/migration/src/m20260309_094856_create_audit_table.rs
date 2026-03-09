use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Actor::Type)
                    .values([Actor::User, Actor::Agent, Actor::System])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Action::Type)
                    .values([
                        Action::SubmitToolCall,
                        Action::EvaluatePolicy,
                        Action::ExecuteTool,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Decision::Type)
                    .values([Decision::Allow, Decision::Deny, Decision::Escalate])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(ReasonCode::Type)
                    .values([
                        ReasonCode::PolicyDenied,
                        ReasonCode::PolicyInputInvalid,
                        ReasonCode::SandboxTimeout,
                        ReasonCode::SandboxResourceExceeded,
                        ReasonCode::SandboxFsViolation,
                        ReasonCode::SandboxNetworkViolation,
                        ReasonCode::RuntimeInternalError,
                        ReasonCode::UnknownFallback,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AuditEvents::Table)
                    .if_not_exists()
                    .col(uuid(AuditEvents::EventId).not_null().primary_key())
                    .col(uuid(AuditEvents::TaskId).not_null())
                    .col(uuid(AuditEvents::StepId).not_null())
                    .col(
                        ColumnDef::new(AuditEvents::Actor)
                            .enumeration(Actor::Type, [Actor::User, Actor::Agent, Actor::System])
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuditEvents::Action)
                            .enumeration(
                                Action::Type,
                                [
                                    Action::SubmitToolCall,
                                    Action::EvaluatePolicy,
                                    Action::ExecuteTool,
                                ],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuditEvents::Decision)
                            .enumeration(
                                Decision::Type,
                                [Decision::Allow, Decision::Deny, Decision::Escalate],
                            )
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(AuditEvents::Timestamp)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(AuditEvents::ReasonCode).enumeration(
                        ReasonCode::Type,
                        [
                            ReasonCode::PolicyDenied,
                            ReasonCode::PolicyInputInvalid,
                            ReasonCode::SandboxTimeout,
                            ReasonCode::SandboxResourceExceeded,
                            ReasonCode::SandboxFsViolation,
                            ReasonCode::SandboxNetworkViolation,
                            ReasonCode::RuntimeInternalError,
                            ReasonCode::UnknownFallback,
                        ],
                    ))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuditEvents::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(ReasonCode::Type).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Decision::Type).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Action::Type).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Actor::Type).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum AuditEvents {
    Table,

    EventId,
    TaskId,
    StepId,
    Actor,
    Action,
    Decision,
    Timestamp,
    ReasonCode,
}

#[derive(DeriveIden)]
enum Actor {
    #[sea_orm(iden = "actor_enum")]
    Type,
    User,
    Agent,
    System,
}

#[derive(DeriveIden)]
enum Action {
    #[sea_orm(iden = "action_enum")]
    Type,
    SubmitToolCall,
    EvaluatePolicy,
    ExecuteTool,
}

#[derive(DeriveIden)]
enum Decision {
    #[sea_orm(iden = "decision_enum")]
    Type,
    Allow,
    Deny,
    Escalate,
}

#[derive(DeriveIden)]
enum ReasonCode {
    #[sea_orm(iden = "reason_code_enum")]
    Type,
    PolicyDenied,
    PolicyInputInvalid,
    SandboxTimeout,
    SandboxResourceExceeded,
    SandboxFsViolation,
    SandboxNetworkViolation,
    RuntimeInternalError,
    UnknownFallback,
}
