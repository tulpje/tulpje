use std::sync::Arc;

use pkrs_fork::{client::PkClient, model::Member};
use tulpje_framework::Error;
use twilight_http::Client;
use uuid::Uuid;

use crate::{
    db::ModPkSystem,
    fronters::{
        db,
        shared::{Fronters, GetSystemFrontersError, get_system_fronters},
    },
};

#[derive(Debug, PartialEq)]
/// minimal system member
struct FrontMember {
    uuid: Uuid,
    name: String,
}

impl From<Member> for FrontMember {
    fn from(value: Member) -> Self {
        Self {
            uuid: value.uuid,
            name: value.display_name.unwrap_or(value.name),
        }
    }
}

/// holds the state of a system's front
#[derive(Debug, PartialEq)]
enum FrontState {
    /// the system doesn't exist in PluralKit (anymore)
    SystemNotFound,
    /// system's front is private
    Private,
    /// system has no switches registered
    None,
    /// system has fronters
    Members(Vec<FrontMember>),
}

impl TryFrom<Result<Option<Fronters>, GetSystemFrontersError>> for FrontState {
    type Error = Error;

    fn try_from(
        value: Result<Option<Fronters>, GetSystemFrontersError>,
    ) -> Result<Self, Self::Error> {
        match value {
            Ok(Some(fronters)) => Ok(Self::Members(
                fronters.members.into_iter().map(Into::into).collect(),
            )),
            Ok(None) => Ok(Self::None),
            Err(GetSystemFrontersError::Private(_)) => Ok(Self::Private),
            Err(GetSystemFrontersError::NotFound(_)) => Ok(Self::SystemNotFound),
            Err(err) => Err(err.into()),
        }
    }
}

async fn update_fronters_for_system(
    db: &sqlx::PgPool,
    pk: &PkClient,
    discord: &Arc<Client>,
    system: &ModPkSystem,
) -> Result<(), Error> {
    // TODO: Fetch from database
    let old_state = FrontState::None;
    let new_state: FrontState = get_system_fronters(pk, system.uuid).await.try_into()?;

    // if state is still the same only update the timestamp
    if old_state == new_state {
        tracing::debug!("fronters unchanged for system {}", system.uuid);
        db::update_fronters_timestamp(db, system.uuid).await?;
        return Ok(());
    }

    tracing::debug!("fronters changed for system {}", system.uuid);

    // TODO: Change update_fronters to accomodate new state structure and
    //       call it here
    // db::update_front_state(db, system.uuid, new_state)
    match new_state {
        FrontState::Members(members) => {
            // process_front_change(db, discord, system, members).await?;
        }
        FrontState::None => {
            // process_front_change(db, discord, system, Vec::new()).await?;
        }
        FrontState::Private => {
            // process_front_private(db, discord, system).await?;
        }
        FrontState::SystemNotFound => {
            // process_system_not_found(db, discord, system).await?;
        }
    }

    Ok(())
}
