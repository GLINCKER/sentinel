## Description

<!-- Provide a clear and concise description of your changes -->

## Type of Change

<!-- Mark all that apply with an "x" -->

- [ ] 🐛 Bug fix (non-breaking change which fixes an issue)
- [ ] ✨ New feature (non-breaking change which adds functionality)
- [ ] 💥 Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] 📝 Documentation update
- [ ] ♻️ Refactor (no functional changes)
- [ ] ⚡ Performance improvement
- [ ] ✅ Test update
- [ ] 🎨 UI/UX improvement

## Related Issues

<!-- Link related issues using keywords: Closes #123, Fixes #456, Refs #789 -->

Closes #

## Changes Made

<!-- List the key changes in this PR with bullet points -->

-
-
-

## Screenshots/Videos (if applicable)

<!-- Add screenshots or screen recordings for UI/UX changes -->

**Before:**
<!-- Screenshot/description of current behavior -->

**After:**
<!-- Screenshot/description of new behavior -->

## Testing

**Test Configuration:**
- **OS:** [e.g., macOS 14.0, Ubuntu 22.04, Windows 11]
- **Rust version:** [e.g., 1.88.0]
- **Node version:** [e.g., 20.10.0]

**Tests Performed:**
- [ ] Unit tests pass (`cargo test`)
- [ ] Integration tests pass (`cargo test --test '*'`)
- [ ] Security tests pass (`cargo test --test security_tests`)
- [ ] Frontend tests pass (`npm test`)
- [ ] E2E tests pass (if applicable)
- [ ] Manual testing completed
- [ ] Tested on multiple platforms (if cross-platform change)

**How to Test:**
<!-- Provide step-by-step instructions for reviewers to test your changes -->

1.
2.
3.

## Code Quality Checklist

- [ ] Code follows Rust/TypeScript style guidelines
- [ ] Ran `cargo fmt` and `cargo clippy -- -D warnings`
- [ ] Ran `npm run format` and `npm run lint` (if frontend changes)
- [ ] Self-review completed (read my own diff)
- [ ] Comments added for complex/non-obvious logic
- [ ] No hardcoded credentials or sensitive data
- [ ] GLINR branding standards followed (see [docs/claude.md](../docs/claude.md))
  - [ ] Component names use `Glinr` prefix (if applicable)
  - [ ] File headers include copyright notice
  - [ ] Package references use `@glinr/` namespace

## Documentation

- [ ] Code comments added/updated (Rustdoc, JSDoc)
- [ ] User-facing documentation updated (README, docs/)
- [ ] CHANGELOG.md updated (if applicable)
- [ ] API documentation updated (if API changes)
- [ ] Examples updated (if config format changed)

## Test Coverage

- [ ] New tests added for new features
- [ ] Coverage maintained at 90%+ (run `cargo llvm-cov`)
- [ ] All edge cases tested
- [ ] Error handling tested
- [ ] Security implications tested (see [docs/TESTING.md](../docs/TESTING.md))

## Performance Impact

<!-- If this PR affects performance, describe the impact -->

- [ ] No performance impact
- [ ] Performance improved (provide benchmarks)
- [ ] Performance impact acceptable (explain why)
- [ ] Benchmarks added/updated (`cargo bench`)

**Benchmarks:**
```
# Paste benchmark results here (if applicable)
```

## Breaking Changes

<!-- If this is a breaking change, describe the impact and provide migration guide -->

**Impact:**
<!-- What breaks? Who is affected? -->

**Migration Guide:**
<!-- Step-by-step instructions for users to migrate -->

1.
2.
3.

## Security Considerations

<!-- Have you considered security implications of this change? -->

- [ ] No security impact
- [ ] Security implications reviewed
- [ ] Input validation added
- [ ] No new dependencies with known vulnerabilities (`cargo audit`)
- [ ] Secrets/credentials properly handled

## Additional Notes

<!-- Any additional information reviewers should know -->

## Checklist for Reviewers

**Code Quality:**
- [ ] Code is clean, readable, and well-structured
- [ ] Naming follows conventions
- [ ] No unnecessary complexity
- [ ] Error handling is appropriate

**Functionality:**
- [ ] Changes work as described
- [ ] Edge cases handled
- [ ] No regressions introduced

**Testing:**
- [ ] Test coverage is adequate
- [ ] Tests are meaningful (not just for coverage)
- [ ] Tests pass locally

**Documentation:**
- [ ] User-facing changes documented
- [ ] Code comments are helpful
- [ ] README/docs updated if needed

**Security:**
- [ ] No security vulnerabilities introduced
- [ ] Input validation present
- [ ] No hardcoded secrets

**Performance:**
- [ ] No performance regressions
- [ ] Benchmarks provided (if applicable)

---

**Built with ❤️ by Glincker (A GLINR Product)**

Thank you for contributing to Sentinel! 🛡️
