#!/bin/bash
set -e
cd frontend
trunk build
mkdir -p ../backend/static
cp -r dist/* ../backend/static/
chmod -R 755 ../backend/static
cd ../backend
shuttle run