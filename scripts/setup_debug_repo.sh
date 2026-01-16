#!/usr/bin/env bash

set -e

# Setup repo
cd /Users/martin/code/_debug/sample-git
git init
git config user.email "bot@example.com"
git config user.name "Test Bot"

# 1. Structure
mkdir -p src/main/java/com/example/legacy
mkdir -p src/lib/utils/helpers
mkdir -p tests/unit/mocks
mkdir -p tests/integration/scenarios/auth
mkdir -p config/docker
mkdir -p build/artifacts
mkdir -p plugins/custom/xml_parser
mkdir -p docs/api/v1
mkdir -p deeply/nested/folder/structure/that/goes/on/forever

# 2. Base files (Initial Commit)
echo "# Project" > README.md
echo "public class Main {}" > src/main/java/com/example/Main.java
echo "test=true" > config/app.ini
touch src/lib/utils/helpers/StringUtil.php
touch docs/index.md
git add .
git commit -m "Initial commit"

# 3. Create Changes (The "Messy" State)

# MODIFIED (Small Diff)
echo "# Project - Updated" > README.md

# MODIFIED (Large Diff)
for i in {1..50}; do echo "log_line_$i = 'data'" >> config/app.ini; done

# ADDED (Deeply Nested)
echo "secret" > deeply/nested/folder/structure/that/goes/on/forever/secret_key.pem

# ADDED (Various Extensions)
echo "<?php phpinfo(); ?>" > src/lib/utils/helpers/Debug.php
echo "console.log('hello')" > plugins/custom/index.js
echo "def main(): pass" > plugins/custom/script.py

# DELETED
rm src/main/java/com/example/Main.java

# RENAMED
git mv docs/index.md docs/home.md

# UNTRACKED
touch TODO.txt
touch notes.log

# STAGED vs UNSTAGED
echo "staged change" >> tests/unit/mocks/MockUser.java
git add tests/unit/mocks/MockUser.java
echo "unstaged change" >> tests/unit/mocks/MockUser.java

echo "Done! Test repo created at $(pwd)"
