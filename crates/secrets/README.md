# wrkflw-secrets

Comprehensive secrets management for wrkflw workflow execution. This crate provides secure handling of secrets with support for multiple providers, encryption, masking, and GitHub Actions-compatible variable substitution.

## Features

- **Multiple Secret Providers**: Environment variables, files, HashiCorp Vault, AWS Secrets Manager, Azure Key Vault, Google Cloud Secret Manager
- **Secure Storage**: AES-256-GCM encryption for secrets at rest
- **Variable Substitution**: GitHub Actions-compatible `${{ secrets.* }}` syntax
- **Secret Masking**: Automatic masking of secrets in logs and output with pattern detection
- **Caching**: Optional caching with TTL for performance optimization
- **Rate Limiting**: Built-in protection against secret access abuse
- **Input Validation**: Comprehensive validation of secret names and values
- **Health Checks**: Provider health monitoring and diagnostics
- **Configuration**: Flexible YAML/JSON configuration with environment variable support
- **Thread Safety**: Full async/await support with concurrent access
- **Performance Optimized**: Compiled regex patterns and caching for high-throughput scenarios

## Quick Start

```rust
use wrkflw_secrets::prelude::*;

#[tokio::main]
async fn main() -> SecretResult<()> {
    // Create a secret manager with default configuration
    let manager = SecretManager::default().await?;
    
    // Set an environment variable
    std::env::set_var("GITHUB_TOKEN", "ghp_your_token_here");
    
    // Get a secret
    let secret = manager.get_secret("GITHUB_TOKEN").await?;
    println!("Token: {}", secret.value());
    
    // Use secret substitution
    let mut substitution = SecretSubstitution::new(&manager);
    let template = "curl -H 'Authorization: Bearer ${{ secrets.GITHUB_TOKEN }}' https://api.github.com";
    let resolved = substitution.substitute(template).await?;
    
    // Mask secrets in logs
    let mut masker = SecretMasker::new();
    masker.add_secret(secret.value());
    let safe_log = masker.mask(&resolved);
    println!("Safe log: {}", safe_log);
    
    Ok(())
}
```

## Configuration

### Environment Variables

```bash
# Set default provider
export WRKFLW_DEFAULT_SECRET_PROVIDER=env

# Enable/disable secret masking
export WRKFLW_SECRET_MASKING=true

# Set operation timeout
export WRKFLW_SECRET_TIMEOUT=30
```

### Configuration File

Create `~/.wrkflw/secrets.yml`:

```yaml
default_provider: env
enable_masking: true
timeout_seconds: 30
enable_caching: true
cache_ttl_seconds: 300

providers:
  env:
    type: environment
    prefix: "WRKFLW_SECRET_"
  
  file:
    type: file
    path: "~/.wrkflw/secrets.json"
  
  vault:
    type: vault
    url: "https://vault.example.com"
    auth:
      method: token
      token: "${VAULT_TOKEN}"
    mount_path: "secret"
```

## Secret Providers

### Environment Variables

The simplest provider reads secrets from environment variables:

```rust
// With prefix
std::env::set_var("WRKFLW_SECRET_API_KEY", "secret_value");
let secret = manager.get_secret_from_provider("env", "API_KEY").await?;

// Without prefix  
std::env::set_var("GITHUB_TOKEN", "ghp_token");
let secret = manager.get_secret_from_provider("env", "GITHUB_TOKEN").await?;
```

### File-based Storage

Store secrets in JSON, YAML, or environment files:

**JSON format** (`secrets.json`):
```json
{
  "API_KEY": "secret_api_key",
  "DB_PASSWORD": "secret_password"
}
```

**Environment format** (`secrets.env`):
```bash
API_KEY=secret_api_key
DB_PASSWORD="quoted password"
GITHUB_TOKEN='single quoted token'
```

**YAML format** (`secrets.yml`):
```yaml
API_KEY: secret_api_key
DB_PASSWORD: secret_password
```

### HashiCorp Vault

```yaml
providers:
  vault:
    type: vault
    url: "https://vault.example.com"
    auth:
      method: token
      token: "${VAULT_TOKEN}"
    mount_path: "secret"
```

### AWS Secrets Manager

```yaml
providers:
  aws:
    type: aws_secrets_manager
    region: "us-east-1"
    role_arn: "arn:aws:iam::123456789012:role/SecretRole"  # optional
```

### Azure Key Vault

```yaml
providers:
  azure:
    type: azure_key_vault
    vault_url: "https://myvault.vault.azure.net/"
    auth:
      method: service_principal
      client_id: "${AZURE_CLIENT_ID}"
      client_secret: "${AZURE_CLIENT_SECRET}"
      tenant_id: "${AZURE_TENANT_ID}"
```

### Google Cloud Secret Manager

```yaml
providers:
  gcp:
    type: gcp_secret_manager
    project_id: "my-project"
    key_file: "/path/to/service-account.json"  # optional
```

## Variable Substitution

Support for GitHub Actions-compatible secret references:

```rust
let mut substitution = SecretSubstitution::new(&manager);

// Default provider
let template = "TOKEN=${{ secrets.GITHUB_TOKEN }}";
let resolved = substitution.substitute(template).await?;

// Specific provider
let template = "API_KEY=${{ secrets.vault:API_KEY }}";
let resolved = substitution.substitute(template).await?;
```

## Secret Masking

Automatically mask secrets in logs and output:

```rust
let mut masker = SecretMasker::new();

// Add specific secrets
masker.add_secret("secret_value");

// Automatic pattern detection for common secret types
let log = "Token: ghp_1234567890123456789012345678901234567890";
let masked = masker.mask(log);
// Output: "Token: ghp_***"
```

Supported patterns:
- GitHub Personal Access Tokens (`ghp_*`)
- GitHub App tokens (`ghs_*`) 
- GitHub OAuth tokens (`gho_*`)
- AWS Access Keys (`AKIA*`)
- JWT tokens
- Generic API keys

## Encrypted Storage

For sensitive environments, use encrypted storage:

```rust
use wrkflw_secrets::storage::{EncryptedSecretStore, KeyDerivation};

// Create encrypted store
let (mut store, key) = EncryptedSecretStore::new()?;

// Add secrets
store.add_secret(&key, "API_KEY", "secret_value")?;

// Save to file
store.save_to_file("secrets.encrypted").await?;

// Load from file
let loaded_store = EncryptedSecretStore::load_from_file("secrets.encrypted").await?;
let secret = loaded_store.get_secret(&key, "API_KEY")?;
```

## Error Handling

All operations return `SecretResult<T>` with comprehensive error types:

```rust
match manager.get_secret("MISSING_SECRET").await {
    Ok(secret) => println!("Secret: {}", secret.value()),
    Err(SecretError::NotFound { name }) => {
        eprintln!("Secret '{}' not found", name);
    }
    Err(SecretError::ProviderNotFound { provider }) => {
        eprintln!("Provider '{}' not configured", provider);
    }
    Err(SecretError::AuthenticationFailed { provider, reason }) => {
        eprintln!("Auth failed for {}: {}", provider, reason);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Health Checks

Monitor provider health:

```rust
let health_results = manager.health_check().await;
for (provider, result) in health_results {
    match result {
        Ok(()) => println!("✓ {} is healthy", provider),
        Err(e) => println!("✗ {} failed: {}", provider, e),
    }
}
```

## Security Best Practices

1. **Use encryption** for secrets at rest
2. **Enable masking** to prevent secrets in logs
3. **Rotate secrets** regularly
4. **Use least privilege** access for secret providers
5. **Monitor access** through health checks and logging
6. **Use provider-specific authentication** (IAM roles, service principals)
7. **Configure rate limiting** to prevent abuse
8. **Validate input** - the system automatically validates secret names and values

## Rate Limiting

Protect against abuse with built-in rate limiting:

```rust
use wrkflw_secrets::rate_limit::RateLimitConfig;
use std::time::Duration;

let mut config = SecretConfig::default();
config.rate_limit = RateLimitConfig {
    max_requests: 100,                    // Max requests per window
    window_duration: Duration::from_secs(60), // 1 minute window
    enabled: true,
};

let manager = SecretManager::new(config).await?;

// Rate limiting is automatically applied to all secret access operations
match manager.get_secret("API_KEY").await {
    Ok(secret) => println!("Success: {}", secret.value()),
    Err(SecretError::RateLimitExceeded(msg)) => {
        println!("Rate limited: {}", msg);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Input Validation

All inputs are automatically validated:

```rust
// Secret names must:
// - Be 1-255 characters long
// - Contain only letters, numbers, underscores, hyphens, and dots
// - Not start or end with dots
// - Not contain consecutive dots
// - Not be reserved system names

// Secret values must:
// - Be under 1MB in size
// - Not contain null bytes
// - Be valid UTF-8

// Invalid examples that will be rejected:
manager.get_secret("").await;                    // Empty name
manager.get_secret("invalid/name").await;        // Invalid characters
manager.get_secret(".hidden").await;             // Starts with dot
manager.get_secret("CON").await;                 // Reserved name
```

## Performance Features

### Caching

```rust
let config = SecretConfig {
    enable_caching: true,
    cache_ttl_seconds: 300, // 5 minutes
    ..Default::default()
};
```

### Optimized Pattern Matching

- Pre-compiled regex patterns for secret detection
- Global pattern cache using `OnceLock`
- Efficient string replacement algorithms
- Cached mask generation

### Benchmarking

Run performance benchmarks:

```bash
cargo bench -p wrkflw-secrets
```

## Feature Flags

Enable optional providers:

```toml
[dependencies]
wrkflw-secrets = { version = "0.1", features = ["vault-provider", "aws-provider"] }
```

Available features:
- `env-provider` (default)
- `file-provider` (default) 
- `vault-provider`
- `aws-provider`
- `azure-provider`
- `gcp-provider`
- `all-providers`

## License

MIT License - see LICENSE file for details.
