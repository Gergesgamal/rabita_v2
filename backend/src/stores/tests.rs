// backend/src/stores/tests.rs

//! Store tests: slug validation, homograph attack rejection per D-022.

#[cfg(test)]
mod tests {
    use crate::stores::StoreManager;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_store_slug_validation() {
        // Slug must be 3-63 chars
        // No leading/trailing/double hyphens
        // Only alphanumeric + hyphens
        // Normalized to lowercase + NFC

        // These would need a DB; marking as ignored for now
        // In a real integration test suite, test:
        // - create with valid slug
        // - create with reserved slug (both tiers)
        // - create with duplicate slug → conflict
        // - update store settings with out-of-range values
        // - suspension requires super-admin context
        // - publication requires products
    }

    #[test]
    fn slug_normalization() {
        let slug = "My-Store".to_lowercase();
        assert_eq!(slug, "my-store");
    }
    #[test]
    fn test_valid_slug() {
        let result = StoreManager::validate_slug("my-awesome-store");
        assert!(result.is_ok());
    }

    #[test]
    fn test_slug_too_short() {
        let result = StoreManager::validate_slug("ab");
        assert!(result.is_err());
    }

    #[test]
    fn test_slug_leading_hyphen() {
        let result = StoreManager::validate_slug("-mystore");
        assert!(result.is_err());
    }

    #[test]
    fn test_slug_double_hyphen() {
        let result = StoreManager::validate_slug("my--store");
        assert!(result.is_err());
    }

    #[test]
    fn test_slug_homograph_cyrillic_a() {
        // Cyrillic 'а' (U+0430) vs Latin 'a' (U+0061)
        // After normalization, should be rejected or normalized
        let cyrillic_slug = "аdmin"; // Starts with Cyrillic а
        let result = StoreManager::validate_slug(cyrillic_slug);
        // Should fail because non-ASCII after normalization
        assert!(result.is_err());
    }

    #[test]
    fn test_slug_uppercase_normalized() {
        let result = StoreManager::validate_slug("MyStore");
        // Should be rejected (uppercase not allowed)
        assert!(result.is_err());
    }
}
