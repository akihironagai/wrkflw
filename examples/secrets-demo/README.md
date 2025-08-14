# wrkflw Secrets Management Demo

This demo demonstrates the comprehensive secrets management system in wrkflw, addressing the critical need for secure secret handling in CI/CD workflows.

## The Problem

Without proper secrets support, workflows are severely limited because:

1. **No way to access sensitive data** - API keys, tokens, passwords, certificates
2. **Security risks** - Hardcoded secrets in code or plain text in logs  
3. **Limited usefulness** - Can't integrate with real services that require authentication
4. **Compliance issues** - Unable to meet security standards for production workflows

## The Solution

wrkflw now provides comprehensive secrets management with:

- **Multiple secret providers** (environment variables, files, HashiCorp Vault, AWS Secrets Manager, etc.)
- **GitHub Actions-compatible syntax** (`${{ secrets.* }}`)
- **Automatic secret masking** in logs and output
- **Encrypted storage** for sensitive environments
- **Flexible configuration** for different deployment scenarios

## Quick Start

### 1. Environment Variables (Simplest)

```bash
# Set secrets as environment variables
export GITHUB_TOKEN="ghp_your_token_here"
export API_KEY="your_api_key"
export DB_PASSWORD="secure_password"
```

Create a workflow that uses secrets:

```yaml
# .github/workflows/secrets-demo.yml
name: Secrets Demo
on: [push]

jobs:
  test-secrets:
    runs-on: ubuntu-latest
    steps:
      - name: Use GitHub Token
        run: |
          echo "Using token to access GitHub API"
          curl -H "Authorization: Bearer ${{ secrets.GITHUB_TOKEN }}" \
            https://api.github.com/user
      
      - name: Use API Key
        run: |
          echo "API Key: ${{ secrets.API_KEY }}"
          
      - name: Database Connection
        env:
          DB_PASS: ${{ secrets.DB_PASSWORD }}
        run: |
          echo "Connecting to database with password: ${DB_PASS}"
```

Run with wrkflw:

```bash
wrkflw run .github/workflows/secrets-demo.yml
```

### 2. File-based Secrets

Create a secrets file:

```json
{
  "API_KEY": "your_api_key_here",
  "DB_PASSWORD": "secure_database_password",
  "GITHUB_TOKEN": "ghp_your_github_token"
}
```

Or environment file format:

```bash
# secrets.env
API_KEY=your_api_key_here
DB_PASSWORD="secure database password"
GITHUB_TOKEN=ghp_your_github_token
```

Configure wrkflw to use file-based secrets:

```yaml
# ~/.wrkflw/secrets.yml
default_provider: file
enable_masking: true
timeout_seconds: 30

providers:
  file:
    type: file
    path: "./secrets.json"  # or "./secrets.env"
```

### 3. Advanced Configuration

For production environments, use external secret managers:

```yaml
# ~/.wrkflw/secrets.yml
default_provider: vault
enable_masking: true
timeout_seconds: 30
enable_caching: true
cache_ttl_seconds: 300

providers:
  env:
    type: environment
    prefix: "WRKFLW_SECRET_"
  
  vault:
    type: vault
    url: "https://vault.company.com"
    auth:
      method: token
      token: "${VAULT_TOKEN}"
    mount_path: "secret"
  
  aws:
    type: aws_secrets_manager
    region: "us-east-1"
    role_arn: "arn:aws:iam::123456789012:role/SecretRole"
```

## Secret Providers

### Environment Variables

**Best for**: Development and simple deployments

```bash
# With prefix
export WRKFLW_SECRET_API_KEY="your_key"
export WRKFLW_SECRET_DB_PASSWORD="password"

# Direct environment variables
export GITHUB_TOKEN="ghp_token"
export API_KEY="key_value"
```

Use in workflows:
```yaml
steps:
  - name: Use prefixed secret
    run: echo "API: ${{ secrets.env:API_KEY }}"
  
  - name: Use direct secret
    run: echo "Token: ${{ secrets.GITHUB_TOKEN }}"
```

### File-based Storage

**Best for**: Local development and testing

Supports multiple formats:

**JSON** (`secrets.json`):
```json
{
  "GITHUB_TOKEN": "ghp_your_token",
  "API_KEY": "your_api_key",
  "DATABASE_URL": "postgresql://user:pass@localhost/db"
}
```

**YAML** (`secrets.yml`):
```yaml
GITHUB_TOKEN: ghp_your_token
API_KEY: your_api_key
DATABASE_URL: postgresql://user:pass@localhost/db
```

**Environment** (`secrets.env`):
```bash
GITHUB_TOKEN=ghp_your_token
API_KEY=your_api_key
DATABASE_URL="postgresql://user:pass@localhost/db"
```

### HashiCorp Vault

**Best for**: Production environments with centralized secret management

```yaml
providers:
  vault:
    type: vault
    url: "https://vault.company.com"
    auth:
      method: token
      token: "${VAULT_TOKEN}"
    mount_path: "secret/v2"
```

Use vault secrets in workflows:
```yaml
steps:
  - name: Use Vault secret
    run: curl -H "X-API-Key: ${{ secrets.vault:api-key }}" api.service.com
```

### AWS Secrets Manager

**Best for**: AWS-native deployments

```yaml
providers:
  aws:
    type: aws_secrets_manager
    region: "us-east-1"
    role_arn: "arn:aws:iam::123456789012:role/SecretRole"
```

### Azure Key Vault

**Best for**: Azure-native deployments

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

## Secret Masking

wrkflw automatically masks secrets in logs to prevent accidental exposure:

```bash
# Original log:
# "API response: {\"token\": \"ghp_1234567890abcdef\", \"status\": \"ok\"}"

# Masked log:
# "API response: {\"token\": \"ghp_***\", \"status\": \"ok\"}"
```

Automatically detects and masks:
- GitHub Personal Access Tokens (`ghp_*`)
- GitHub App tokens (`ghs_*`)
- GitHub OAuth tokens (`gho_*`)
- AWS Access Keys (`AKIA*`)
- JWT tokens
- Generic API keys

## Workflow Examples

### GitHub API Integration

```yaml
name: GitHub API Demo
on: [push]

jobs:
  github-integration:
    runs-on: ubuntu-latest
    steps:
      - name: List repositories
        run: |
          curl -H "Authorization: Bearer ${{ secrets.GITHUB_TOKEN }}" \
            -H "Accept: application/vnd.github.v3+json" \
            https://api.github.com/user/repos
      
      - name: Create issue
        run: |
          curl -X POST \
            -H "Authorization: Bearer ${{ secrets.GITHUB_TOKEN }}" \
            -H "Accept: application/vnd.github.v3+json" \
            https://api.github.com/repos/owner/repo/issues \
            -d '{"title":"Automated issue","body":"Created by wrkflw"}'
```

### Database Operations

```yaml
name: Database Demo
on: [push]

jobs:
  database-ops:
    runs-on: ubuntu-latest
    steps:
      - name: Run migrations
        env:
          DATABASE_URL: ${{ secrets.DATABASE_URL }}
          DB_PASSWORD: ${{ secrets.DB_PASSWORD }}
        run: |
          echo "Running database migrations..."
          # Your migration commands here
          
      - name: Backup database
        run: |
          pg_dump "${{ secrets.DATABASE_URL }}" > backup.sql
```

### Multi-Provider Example

```yaml
name: Multi-Provider Demo
on: [push]

jobs:
  multi-secrets:
    runs-on: ubuntu-latest
    steps:
      - name: Use environment secret
        run: echo "Env: ${{ secrets.env:API_KEY }}"
      
      - name: Use file secret
        run: echo "File: ${{ secrets.file:GITHUB_TOKEN }}"
      
      - name: Use Vault secret
        run: echo "Vault: ${{ secrets.vault:database-password }}"
      
      - name: Use AWS secret
        run: echo "AWS: ${{ secrets.aws:prod/api/key }}"
```

## Security Best Practices

### 1. Use Appropriate Providers

- **Development**: Environment variables or files
- **Staging**: File-based or simple vault
- **Production**: External secret managers (Vault, AWS, Azure, GCP)

### 2. Enable Secret Masking

Always enable masking in production:

```yaml
enable_masking: true
```

### 3. Rotate Secrets Regularly

Use secret managers that support automatic rotation:

```yaml
providers:
  aws:
    type: aws_secrets_manager
    region: "us-east-1"
    # AWS Secrets Manager handles automatic rotation
```

### 4. Use Least Privilege

Grant minimal necessary permissions:

```yaml
providers:
  vault:
    type: vault
    auth:
      method: app_role
      role_id: "${VAULT_ROLE_ID}"
      secret_id: "${VAULT_SECRET_ID}"
    # Role has access only to required secrets
```

### 5. Monitor Secret Access

Use secret managers with audit logging:

```yaml
providers:
  azure:
    type: azure_key_vault
    vault_url: "https://myvault.vault.azure.net/"
    # Azure Key Vault provides detailed audit logs
```

## Troubleshooting

### Secret Not Found

```bash
Error: Secret 'API_KEY' not found

# Check:
1. Secret exists in the provider
2. Provider is correctly configured  
3. Authentication is working
4. Correct provider name in ${{ secrets.provider:name }}
```

### Authentication Failed

```bash
Error: Authentication failed for provider 'vault'

# Check:
1. Credentials are correct
2. Network connectivity to secret manager
3. Permissions for the service account
4. Token/credential expiration
```

### Secret Masking Not Working

```bash
# Secrets appearing in logs

# Check:
1. enable_masking: true in configuration
2. Secret is properly retrieved (not hardcoded)
3. Secret matches known patterns for auto-masking
```

## Migration Guide

### From GitHub Actions

Most GitHub Actions workflows work without changes:

```yaml
# This works directly in wrkflw
steps:
  - name: Deploy
    env:
      API_TOKEN: ${{ secrets.API_TOKEN }}
    run: deploy.sh
```

### From Environment Variables

```bash
# Before (environment variables)
export API_KEY="your_key"
./script.sh

# After (wrkflw secrets)
# Set in secrets.env:
# API_KEY=your_key

# Use in workflow:
# ${{ secrets.API_KEY }}
```

### From CI/CD Platforms

Most secrets can be migrated by:

1. Exporting from current platform
2. Importing into wrkflw's chosen provider
3. Updating workflow syntax to `${{ secrets.NAME }}`

## Performance Considerations

### Caching

Enable caching for frequently accessed secrets:

```yaml
enable_caching: true
cache_ttl_seconds: 300  # 5 minutes
```

### Connection Pooling

For high-volume deployments, secret managers support connection pooling:

```yaml
providers:
  vault:
    type: vault
    # Vault client automatically handles connection pooling
```

### Timeout Configuration

Adjust timeouts based on network conditions:

```yaml
timeout_seconds: 30  # Increase for slow networks
```

## Conclusion

With comprehensive secrets management, wrkflw is now suitable for production workflows requiring secure access to:

- External APIs and services
- Databases and storage systems
- Cloud provider resources
- Authentication systems
- Deployment targets

The flexible provider system ensures compatibility with existing secret management infrastructure while providing a GitHub Actions-compatible developer experience.

**The usefulness limitation has been removed** - wrkflw can now handle real-world CI/CD scenarios securely and efficiently.
