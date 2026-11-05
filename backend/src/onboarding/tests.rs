// backend/src/onboarding/tests.rs

//! Onboarding tests.

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_generate_credentials() {
        let (identifier, password) = provisioning::ProvisioningManager::generate_credentials();

        assert!(identifier.starts_with("mer-"));
        assert_eq!(identifier.len(), 16); // "mer-" + 12 chars
        assert!(password.len() >= 12 && password.len() <= 16);

        // Verify no ambiguous chars
        assert!(!identifier.contains('L'));
        assert!(!identifier.contains('I'));
        assert!(!identifier.contains('O'));
        assert!(!identifier.contains('U'));
    }

    #[test]
    fn test_generate_initial_code() {
        let code = provisioning::ProvisioningManager::generate_initial_code();

        assert_eq!(code.len(), 8);
        assert!(code.parse::<u64>().is_ok());
        let num = code.parse::<u64>().unwrap();
        assert!(num >= 10000000 && num <= 99999999);
    }

    #[tokio::test]
    #[ignore]
    async fn test_application_submission_rate_limits() {
        // TODO: Test IP and email rate limits per D-039
    }

    #[tokio::test]
    #[ignore]
    async fn test_application_email_verification() {
        // TODO: Test verification flow
    }

    #[tokio::test]
    #[ignore]
    async fn test_merchant_provisioning() {
        // TODO: Test credential generation and account creation
    }
}
