use uuid::Uuid;

use crate::domain::user::model::{Role, User};

/// Trait for resources that have an owner (user_id)
pub trait OwnedResource {
    /// Returns the UUID of the user who owns this resource
    fn owner_id(&self) -> Uuid;
}

/// Generic authorization check for owned resources
/// Returns true if the authenticated user can access/modify the resource
///
/// Access is granted if:
/// - User is an Admin (can access all resources), OR
/// - User is the owner of the resource
pub fn can_access_resource<T: OwnedResource>(authenticated_user: &User, resource: &T) -> bool {
    authenticated_user.role == Role::Admin || authenticated_user.id == resource.owner_id()
}

/// Returns an ownership filter for queries
///
/// - If user is Admin: Returns None (no filter, see all resources)
/// - If user is not Admin: Returns Some(user_id) (filter to owned resources)
///
/// Use this for "get all" type operations where admins see everything
/// but regular users only see their own resources.
pub fn get_ownership_filter(authenticated_user: &User) -> Option<Uuid> {
    match authenticated_user.role {
        Role::Admin => None,
        _ => Some(authenticated_user.id),
    }
}

/// Check if user is an admin
pub fn is_admin(user: &User) -> bool {
    user.role == Role::Admin
}

/// Require admin role, returns error message if not admin
pub fn require_admin(user: &User) -> Result<(), String> {
    if is_admin(user) {
        Ok(())
    } else {
        Err("Admin access required".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::model::{Role, User, UserStatus};

    struct TestResource {
        owner: Uuid,
    }

    impl OwnedResource for TestResource {
        fn owner_id(&self) -> Uuid {
            self.owner
        }
    }

    fn create_test_user(id: Uuid, role: Role) -> User {
        User {
            id,
            display_name: Some("Test User".to_string()),
            username: Some("test".to_string()),
            email: Some("test@example.com".to_string()),
            password_hash: "hash".to_string(),
            role,
            status: UserStatus::Active,
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
        }
    }

    #[test]
    fn test_admin_can_access_any_resource() {
        let admin = create_test_user(Uuid::now_v7(), Role::Admin);
        let resource = TestResource {
            owner: Uuid::now_v7(),
        };

        assert!(can_access_resource(&admin, &resource));
    }

    #[test]
    fn test_user_can_access_own_resource() {
        let user_id = Uuid::now_v7();
        let user = create_test_user(user_id, Role::Student);
        let resource = TestResource { owner: user_id };

        assert!(can_access_resource(&user, &resource));
    }

    #[test]
    fn test_user_cannot_access_others_resource() {
        let user = create_test_user(Uuid::now_v7(), Role::Student);
        let resource = TestResource {
            owner: Uuid::now_v7(),
        };

        assert!(!can_access_resource(&user, &resource));
    }

    #[test]
    fn test_admin_gets_no_filter() {
        let admin = create_test_user(Uuid::now_v7(), Role::Admin);
        assert_eq!(get_ownership_filter(&admin), None);
    }

    #[test]
    fn test_user_gets_filtered() {
        let user_id = Uuid::now_v7();
        let user = create_test_user(user_id, Role::Student);
        assert_eq!(get_ownership_filter(&user), Some(user_id));
    }
}
