# TEVM-Agent Ansible Setup

Ansible playbook for quickly provisioning a Hetzner AX41-NVMe server with Docker and Claude Code.

## Specifications

- **Server**: Hetzner AX41-NVMe
- **CPU**: AMD Ryzen 5 3600 (6 cores/12 threads @ 3.6GHz)
- **RAM**: 64GB DDR4 RAM
- **Storage**: 2 x 512GB NVMe SSD
- **Price**: $42.00/month or $0.0643/hour

## Prerequisites

```bash
# 1. Install Ansible
pip install ansible

# 2. Install the Hetzner collection
ansible-galaxy collection install hetzner.hcloud

# 3. Set your Hetzner API token
export HCLOUD_API_TOKEN=your_token_here
```

## Configuration

Before running the playbook, update the following in `playbook.yml`:

1. Modify the `ssh_keys` variable to include your SSH key name as registered in Hetzner
2. Change the default root password in the container setup section
3. Adjust any other settings as needed (server location, etc.)

## Usage

```bash
# Deploy the server and configure Docker
ansible-playbook playbook.yml

# SSH into the server
ssh root@<server_ip>

# SSH into the Docker container
ssh -p 2222 root@<server_ip>

# Run multiple Claude Code instances
./launch-claude-code.sh 3  # Launches 3 instances
```

## Features

- Automatically provisions a Hetzner AX41-NVMe server
- Installs Docker and sets up an Ubuntu container
- Configures SSH for remote access to the container
- Includes a script to launch multiple Claude Code instances