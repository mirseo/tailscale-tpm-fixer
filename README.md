# Tailscale TPM Fixer

A utility tool designed to resolve Tailscale TPM (Trusted Platform Module) recognition errors on Windows and Linux systems.

## Overview

This tool helps fix issues that occur when Tailscale's TPM-bound state becomes invalid or inaccessible. These problems typically arise after hardware changes or TPM key management operations, preventing Tailscale from functioning properly.

## When to Use

Use this tool if you encounter TPM-related errors with Tailscale after:

- Replacing your CPU or motherboard
- Performing TPM key management operations
- Resetting or clearing your TPM
- Any scenario where TPM keys have been changed or reset

The tool works by removing Tailscale's TPM-bound state files, allowing you to re-register your device with a fresh configuration.

## Installation

Download the latest pre-built binaries from the GitHub releases page [Github release]().

## Requirements

**Administrator privileges are required** to run this tool. The application needs elevated permissions to:
- Stop running Tailscale processes
- Remove system-level configuration files

### Windows
Run the executable as Administrator (right-click and select "Run as administrator").

### Linux
Run with sudo:
```bash
sudo ./tailscale-tpm-fixer
```

## Usage

1. Close any Tailscale applications if they are running
2. Run the tool with administrator privileges
3. Review the list of files that will be removed
4. Press `Y` to proceed or `N` to cancel
5. After completion, re-register your device with Tailscale

The tool will automatically:
- Stop all running Tailscale processes
- Remove TPM-bound state files
- Provide confirmation when the operation is complete

## Files Removed

### Windows
- `C:\ProgramData\Tailscale`
- `C:\Users\%USERNAME%\AppData\Local\Tailscale`

### Linux
- `/var/lib/tailscale/tailscaled.state`

## After Running

Once the tool completes successfully, you'll need to re-register your device with Tailscale to reactivate it. Your device will be treated as a new node on your network.

For more information about Tailscale's secure node state storage, see the [official documentation](https://tailscale.com/kb/1596/secure-node-state-storage).

## License
This project is provided as-is for users experiencing TPM-related issues with Tailscale.
