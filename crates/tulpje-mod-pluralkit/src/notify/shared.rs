use pluralkit_rs::{
    PluralKit,
    models::{PluralKitError, SystemRef},
};

use tulpje_framework::Error;

use crate::db::{self, ModPkSystem};

// TODO: Fetch from DB first, and only fetch from PK if outdated
pub(super) async fn resolve_system_from_reference(
    system_ref: &SystemRef,
    pk_client: &PluralKit,
    db: &sqlx::PgPool,
) -> Result<Option<ModPkSystem>, Error> {
    match pk_client.get_system(system_ref).await {
        Ok(response) => Ok(Some(response.model().await?.into())),
        Err(PluralKitError::PluralKit { code: 20001, .. }) => match system_ref {
            SystemRef::Id(_) | SystemRef::Uuid(_) => Ok(db::get_system(db, system_ref).await?),
            SystemRef::Snowflake(_) => {
                Err("something went wrong, please try using a system ID instead".into())
            }
            SystemRef::Me => unreachable!(), // TODO: somehow enforce this?
        },
        Err(err) => Err(err.into()),
    }
}
