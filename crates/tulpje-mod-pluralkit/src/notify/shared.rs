use pkrs_fork::{
    client::{PkClient, PluralKitError},
    model::PkId,
};

use tulpje_framework::Error;

use crate::{
    db::{self, ModPkSystem},
    util::SystemRef,
};

// TODO: Fetch from DB first, and only fetch from PK if outdated
pub(super) async fn resolve_system_from_reference(
    system_ref: &SystemRef,
    pk_client: &PkClient,
    db: &sqlx::PgPool,
) -> Result<Option<ModPkSystem>, Error> {
    match pk_client.get_system(&PkId(system_ref.clone().into())).await {
        Ok(system) => Ok(Some(ModPkSystem {
            id: system.id.0,
            uuid: system.uuid,
            name: system.name,
        })),
        Err(PluralKitError::Pk(_, message)) if message.code == 20001 => match system_ref {
            SystemRef::Id(_) | SystemRef::Uuid(_) => Ok(db::get_system(db, system_ref).await?),
            SystemRef::DiscordId(_) => {
                Err("something went wrong, please try using a system ID instead".into())
            }
        },
        Err(err) => Err(err.into()),
    }
}
