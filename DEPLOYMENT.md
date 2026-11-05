# Deployment Guide

## Pre-Deployment

1. **Tests**: All tests pass
2. **Security**: cargo-audit, ZAP scan, PDPL review
3. **Backups**: Manual snapshot taken
4. **Notifications**: Team notified of maintenance window

## Staging Deployment

```bash
git push origin develop
# CI/CD automatically deploys to staging

# Run smoke tests
k6 run load-tests/smoke.js
```

## Production Deployment

### Blue-Green Deployment

1. **Green environment**: New version deployed (parallel)
2. **Testing**: Smoke tests, manual verification
3. **Switch**: Traffic redirected to green
4. **Cleanup**: Blue environment kept for 1 hour (quick rollback)

```bash
git tag v1.0.0
git push origin v1.0.0
# CI/CD builds, deploys green, runs tests, switches traffic
```

### Rollback

```bash
# If issues detected, switch traffic back to blue
aws ecs update-service --cluster rabita-prod --service backend --force-new-deployment

# Or use CodeDeploy rollback
aws deploy create-deployment \
  --application-name rabita \
  --deployment-group-name prod \
  --revision '{"revisionType": "S3", "s3Location": {"bucket": "...", "key": "...", "bundleType": "zip"}}'
```

## Database Migrations

### Zero-Downtime Migrations

1. **Add column**: Null-safe, backwards-compatible
2. **Populate**: Background job
3. **Add constraint**: After data is populated
4. **Remove old**: After deprecation period

### Emergency Rollback

If migration causes issues:

```bash
# Restore from backup (35-day PITR window)
aws rds restore-db-instance-to-point-in-time \
  --source-db-instance-identifier rabita-prod \
  --db-instance-identifier rabita-prod-restore \
  --restore-time 2026-10-28T12:00:00Z
```

## Monitoring Post-Deployment

Watch for:

- Error rate spike > 1%
- Latency p95 > 800ms
- Database CPU > 80%
- Outbox DLQ depth
- Missing API responses
- Consumer complaints

If issues: Use CodeDeploy rollback immediately.

## Release Notes Template

```markdown
## v1.0.0 — 2026-10-28

### New Features
- [ ] Feature description

### Bug Fixes
- [ ] Bug fix description

### Breaking Changes
- [ ] Breaking change and migration path

### Performance
- [ ] Performance improvement description

### Known Issues
- [ ] Known issue and workaround

### Upgrade Path
1. Read breaking changes
2. Backup database
3. Deploy new version
4. Monitor error rate
```
