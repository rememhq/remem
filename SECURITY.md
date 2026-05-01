# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 0.1.x   | ✅ Current release |

## Reporting a Vulnerability

**Please do NOT open a public GitHub issue for security vulnerabilities.**

Instead, report vulnerabilities privately via one of these channels:

1. **GitHub Security Advisory** (preferred):
   - Go to [github.com/rememhq/remem/security/advisories/new](https://github.com/rememhq/remem/security/advisories/new)
   - Fill out the advisory form with as much detail as possible

2. **Email**:
   - Send details to the maintainer via GitHub's private messaging

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Affected versions
- Potential impact
- Suggested fix (if any)

### Response Timeline

| Action                        | Timeline      |
|-------------------------------|---------------|
| Acknowledge receipt           | 48 hours      |
| Initial assessment            | 5 business days |
| Patch development             | 14 business days |
| Public disclosure (coordinated) | After patch release |

## Security Considerations

### API Keys

remem requires API keys for cloud LLM providers (Anthropic, OpenAI). These are sensitive credentials:

- **Never commit API keys** to version control
- Use environment variables (`ANTHROPIC_API_KEY`, `OPENAI_API_KEY`)
- The REST API supports bearer token auth via `REMEM_API_KEY`
- In production, always set `REMEM_API_KEY` to restrict access

### Data Storage

- Memory data is stored locally in SQLite databases
- Vector indices are stored as JSON files on disk
- No data is transmitted except to configured LLM providers
- All LLM API calls use HTTPS/TLS

### Dependency Security

- Dependencies are monitored via GitHub Dependabot
- `cargo audit` is run in CI to check for known vulnerabilities
- The `rusqlite` crate uses a bundled SQLite build to avoid system library issues

## Scope

The following are in scope for security reports:

- Authentication bypass in the REST API
- Memory data leakage between projects/sessions
- Injection attacks via memory content
- Dependency vulnerabilities with exploitable impact
- Insecure default configurations

The following are **out of scope**:

- Denial of service via large memory stores (expected behavior)
- LLM prompt injection via stored memory content (inherent to LLM usage)
- Vulnerabilities in upstream LLM provider APIs
