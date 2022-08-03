use crate::background_jobs::Environment;
use crate::models::Version;
use diesel::prelude::*;
use swirl::PerformError;

// What information do we want to send in the webhook?
// - NewVersionReleased
//  - Crate Name
//  - Crate Version
//  - Published by
//  - Published at
//  -

#[swirl::background_job]
pub fn notify_owners(
    env: &Environment,
    conn: &PgConnection,
    new_version: Version,
) -> Result<(), PerformError> {
    let client = env.http_client();

    client.post("<URL HERE>").json(&new_version).send()?;

    println!("Executing notify_owners job!");
    Ok(())
}
