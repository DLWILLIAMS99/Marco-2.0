# Marco 2.0 - PowerShell Command Reference

## Git Commands (Windows PowerShell)
Use semicolons (`;`) instead of `&&` for command chaining in PowerShell:

### Correct PowerShell Syntax:
```powershell
# Stage and commit
git add -A; git commit -m "Your commit message"

# Stage, commit, and push
git add -A; git commit -m "Your commit message"; git push origin master

# Multiple commands
npm install; npm run build; npm test
```

### Avoid (Linux/Bash syntax):
```bash
# Don't use && in PowerShell
git add -A && git commit -m "Your commit message"  # ❌ Won't work
```

## Common Development Commands

### Node.js/NPM:
```powershell
# Install and build
npm install; npm run build

# Run tests and start
npm test; npm start

# Clean and reinstall
rm -r node_modules; npm install
```

### Docker Commands:
```powershell
# Build and run
docker build -t marco2:latest .; docker run -p 8080:80 marco2:latest

# Stop and remove
docker stop marco2-container; docker rm marco2-container
```

### Rust/Cargo:
```powershell
# Build and test
cargo build --release; cargo test

# Format and check
cargo fmt; cargo clippy
```

## Terminal Notes
- PowerShell is the default shell on Windows
- Use `;` for command chaining
- Commands may take time to complete - wait for prompt return
- Use `Ctrl+C` to cancel long-running commands
