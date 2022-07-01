#!/bin/sh

# Clean
rm -rf .venv
rm -rf build clean test pack publish
rm -rf ./scripts/requirements.txt

# Install virutal environment(python) `.venv`;
python3 -m venv .venv
source .venv/bin/activate

pip3 install pipreqs
pipreqs ./scripts --force

# Install dependencies(python) by `requirements.txt`;
pip3 install -r ./scripts/requirements.txt

# Chmod python scripts;
chmod u+x ./scripts/build.py
chmod u+x ./scripts/clean.py
chmod u+x ./scripts/pack.py
chmod u+x ./scripts/publish.py

# Create softlink to scripts
ln -s ./scripts/build.py build
ln -s ./scripts/clean.py clean
ln -s ./scripts/test.py test
ln -s ./scripts/pack.py pack
ln -s ./scripts/publish.py publish