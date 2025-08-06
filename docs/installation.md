---
title: Installation
nav_order: 2
---

## Installation

## Prerequisites

- Rust ≥ 1.70

## Steps

1. **Install Rust**  
   - **Linux/macOS**:  
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - **Windows**:  
     Download and run the installer from [rustup.rs](https://rustup.rs)

2. **Verify Installation**  
   ```bash
   rustc --version   # Linux/macOS
   ```

   ```powershell
   rustc --version   # Windows
   ```

3. **Install Kerno**  
   ```bash
   cargo install kerno
   ```

4. **Verify Kerno**  
   ```bash
   kerno
   ```

See [Usage](usage.md) for more info.
