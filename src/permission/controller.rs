use rocket::futures::TryStreamExt;
use sqlx::Row;

use crate::core::DbPool;

use super::model::{AllowedActions, AllowedActionsBuilder, Resource, ResourceType};

pub async fn is_user_authorized(
    resource: Resource,
    user_id: i64,
    pool: &DbPool,
) -> Result<AllowedActions, sqlx::Error> {
    let allowed_actions = match resource.rtype {
        ResourceType::Service => {
            get_user_permissions_for_service(user_id, resource.id, pool).await?
        }

        // In debug mode, return default permissions
        // In release mode, there will be an error because
        // Some case aren't explicitely handled
        #[cfg(debug_assertions)]
        _ => {
            println!(
                "WARNING: No permission implementation for resource type {:?}",
                resource.rtype
            );
            AllowedActionsBuilder::default()
                .resource(resource)
                .build()
                .unwrap_or_default()
        }
    };

    Ok(allowed_actions)
}

pub async fn get_user_permissions_for_service(
    user_id: i64,
    service_id: i64,
    pool: &DbPool,
) -> Result<AllowedActions, sqlx::Error> {
    // Query the database and build UserAction
    let mut builder = AllowedActionsBuilder::default();

    // Check each permission type using standard query approach
    let query = "
        SELECT pt.name
        FROM resource_permissions rp
        JOIN permission_types pt ON rp.permission_type_id = pt.id
        WHERE rp.resource_type = 'service'
        AND rp.resource_id = ?
        AND rp.granted_to = 'user'
        AND rp.granted_to_id = ?
    ";

    let mut rows = sqlx::query(query)
        .bind(service_id)
        .bind(user_id)
        .fetch(pool);

    // Process each row and set appropriate flags based on results
    while let Some(row) = rows.try_next().await? {
        let permission_name: String = row.try_get("name")?;

        match permission_name.as_str() {
            "read" => {
                builder.read(true);
            }
            "write" => {
                builder.write(true);
            }
            "create" => {
                builder.create(true);
            }
            "delete" => {
                builder.delete(true);
            }
            _ => {}
        }
    }

    Ok(builder.build().unwrap_or_default())
}
