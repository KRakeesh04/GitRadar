# GitRadar Security Architecture

Version: 1.0

---

# Security Philosophy

GitRadar processes highly sensitive information:

* Source code
* Project structures
* Commit history
* Developer activity

Therefore:

> Everything is denied by default unless explicitly allowed.

---

# Security Goals

Protect:

* User source code
* Repository metadata
* Local filesystem
* Future credentials
* Future Git operations

---

# Security Layers

```text
User
 │
 ▼
React UI
 │
 ▼
Tauri Commands
 │
 ▼
Input Validation Layer
 │
 ▼
Permission Layer
 │
 ▼
Git Sandbox
 │
 ▼
Repository Services
 │
 ▼
SQLite
```

---

# Layer 1: Input Validation

Every command must validate:

* IDs
* Paths
* Query parameters

Example:

BAD

../../../etc/passwd

GOOD

/home/user/projects/app

---

# Layer 2: Root Permission Model

GitRadar must never scan arbitrary folders.

Allowed:

User added:

/home/user/projects

Can access:

/home/user/projects/app1

Can access:

/home/user/projects/app2

Blocked:

/home/user/Documents

Blocked:

/etc

Blocked:

/root

---

# Layer 3: Path Traversal Protection

Before opening any file:

1. Canonicalize path
2. Resolve symlinks
3. Verify path is inside approved root

Example:

Requested:

/projects/app/../../etc/passwd

Resolved:

/etc/passwd

Result:

BLOCKED

---

# Layer 4: Git Sandbox

Allowed Commands

* git status
* git log
* git show
* git diff
* git branch

Future:

* git pull
* git push
* git merge

Blocked:

* shell execution
* bash scripts
* arbitrary commands

Never:

std::process::Command(user_input)

---

# Layer 5: Repository Isolation

Repository A must never affect Repository B.

Every operation requires:

repo_id

Validation:

repo belongs to approved root

---

# Layer 6: Secure Storage

SQLite stores:

* Analytics
* Metadata
* Settings

SQLite must never store:

* Passwords
* Tokens (plain text)
* SSH keys

---

# Layer 7: Secret Storage

Future:

WakaTime
GitHub
GitLab

Store secrets in:

Linux Secret Service

Examples:

GNOME Keyring

KDE Wallet

Never SQLite.

---

# Layer 8: Audit Logging

Log:

ROOT_ADDED

ROOT_REMOVED

SETTINGS_CHANGED

GIT_PULL

GIT_PUSH

WAKATIME_CONNECTED

Store:

timestamp
action
details

---

# Layer 9: Diff Safety

Limit:

Maximum diff size

Example:

20 MB

If larger:

Show warning.

Avoid memory exhaustion.

---

# Layer 10: File Preview Protection

Never load entire files.

Preview:

First 50 KB

Large files:

Load on demand.

---

# Layer 11: Binary File Protection

Detect:

* Images
* Videos
* Archives

Do not index content.

Store metadata only.

---

# Layer 12: Database Protection

Enable:

PRAGMA foreign_keys=ON

PRAGMA journal_mode=WAL

Use:

Prepared statements

Never:

String concatenated SQL

---

# Layer 13: Future Network Security

If internet support added:

Default:

OFF

User must enable.

---

# Security Checklist

✓ Root validation

✓ Path validation

✓ Git sandbox

✓ Secret manager

✓ Audit logs

✓ Prepared SQL

✓ Diff limits

✓ Binary detection

✓ Repository isolation

✓ Network disabled by default
