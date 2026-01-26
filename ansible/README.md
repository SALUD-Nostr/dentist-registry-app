# Ansible Deployment for Salud Dental

This directory contains Ansible playbooks for building and deploying the Salud Dental Yew/Trunk application to Cloudflare Pages.

## Prerequisites

1. **Ansible** installed on your system
   ```bash
   # Ubuntu/Debian
   sudo apt install ansible

   # Fedora/RHEL
   sudo dnf install ansible

   # macOS
   brew install ansible
   ```

2. **Cloudflare Account** with Pages enabled
   - Account ID
   - API Token with Pages write permissions

## Setup

### 1. Set Environment Variables

Export your Cloudflare credentials:

```bash
export CLOUDFLARE_ACCOUNT_ID="your-account-id-here"
export CLOUDFLARE_API_TOKEN="your-api-token-here"
```

Alternatively, create a `.env` file:

```bash
# .env (not committed to git)
CLOUDFLARE_ACCOUNT_ID=your-account-id-here
CLOUDFLARE_API_TOKEN=your-api-token-here
```

Then source it:
```bash
source .env
```

### 2. Get Cloudflare Credentials

**Account ID:**
1. Log in to Cloudflare Dashboard
2. Go to Pages
3. Your Account ID is in the URL or sidebar

**API Token:**
1. Go to Cloudflare Dashboard > My Profile > API Tokens
2. Create a new token with "Cloudflare Pages - Edit" permissions
3. Copy the token (it won't be shown again)

## Usage

### Deploy Dental Intake App

```bash
ansible-playbook ansible/deploy-dental-intake.yml
```

### Deploy with Inline Variables

If you prefer not to use environment variables:

```bash
ansible-playbook ansible/deploy-dental-intake.yml \
  -e "cloudflare_account_id=YOUR_ACCOUNT_ID" \
  -e "cloudflare_api_token=YOUR_API_TOKEN"
```

## What the Playbook Does

The deployment playbook performs the following steps:

1. **Validate Credentials**: Check that Cloudflare credentials are set
2. **Install Dependencies**:
   - Rust toolchain (if not installed)
   - wasm32-unknown-unknown target
   - Trunk (Yew build tool)
   - Node.js and npm
   - Tailwind CSS 4
   - Wrangler CLI
3. **Detect Git Branch**: Automatically detect current git branch for deployment
4. **Clean Build**: Remove previous dist/ directory
5. **Build Styles**: Compile Tailwind CSS
6. **Build Application**: Run `trunk build --release`
7. **Deploy**: Push to Cloudflare Pages using wrangler with the current branch

## Branch-Based Deployments

The playbook automatically detects your current git branch and deploys to that branch in Cloudflare Pages. This prevents accidentally overwriting production:

- **`main` or `mera` branch**: Deploys to production
- **Any other branch**: Deploys to a preview deployment for that branch

Example:
```bash
# On main branch - deploys to production
git checkout main
ansible-playbook ansible/deploy-dental-intake.yml

# On feature branch - deploys to preview
git checkout feature/new-ui
ansible-playbook ansible/deploy-dental-intake.yml
# Creates preview at: https://<branch-name>.<project>.pages.dev
```

This is safer than hardcoded branches and allows you to test features before merging to production.

## Configuration

### Customize Project Name

Edit the playbook variables:

```yaml
vars:
  cloudflare_project_name: "your-custom-project-name"
```

Or pass as command-line argument:

```bash
ansible-playbook ansible/deploy-dental-intake.yml \
  -e "cloudflare_project_name=my-custom-name"
```

### Override Branch Detection

By default, deployments automatically use your current git branch. To override and deploy to a specific branch regardless of your current branch:

Edit the deploy task in the playbook to use a hardcoded value:
```yaml
--branch=production  # or staging, dev, etc.
```

Or checkout the branch you want to deploy from:
```bash
git checkout production
ansible-playbook ansible/deploy-dental-intake.yml
```

## Troubleshooting

### Permission Denied for npm/Tailwind

The playbook installs global npm packages with `sudo`. If you prefer user-local installation, modify the tasks to remove `become: yes` and configure npm prefix:

```bash
npm config set prefix ~/.local
export PATH=~/.local/bin:$PATH
```

### Rust Not in PATH

If Rust tools aren't found after installation, source the cargo environment:

```bash
source $HOME/.cargo/env
```

Then re-run the playbook.

### Wrangler Authentication Issues

Ensure your API token has the correct permissions:
- Account - Cloudflare Pages - Edit

### Build Failures

Check the application builds locally first:

```bash
cd dental_intake
trunk build --release
```

## CI/CD Integration

This playbook can be integrated into CI/CD pipelines:

### GitLab CI Example

```yaml
deploy_dental_intake:
  stage: deploy
  image: ubuntu:latest
  before_script:
    - apt-get update && apt-get install -y ansible
  script:
    - ansible-playbook ansible/deploy-dental-intake.yml
  only:
    - main
```

### GitHub Actions Example

```yaml
name: Deploy to Cloudflare Pages
on:
  push:
    branches: [main, mera]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Ansible
        run: sudo apt-get install -y ansible
      - name: Deploy Dental Intake
        run: ansible-playbook ansible/deploy-dental-intake.yml
        env:
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
```

## Project Structure

```
ansible/
├── deploy-dental-intake.yml  # Dental Intake deployment playbook
└── README.md                 # This file
```

## Additional Resources

- [Trunk Documentation](https://trunkrs.dev/)
- [Cloudflare Pages Docs](https://developers.cloudflare.com/pages/)
- [Wrangler CLI Docs](https://developers.cloudflare.com/workers/wrangler/)
- [Ansible Documentation](https://docs.ansible.com/)
