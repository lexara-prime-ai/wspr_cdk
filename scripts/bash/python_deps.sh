#!/bin/bash

# Update the package list
echo "Updating package list..."
sudo apt-get update

# Install pip if it is not already installed
echo "Checking for pip..."
if ! command -v pip &> /dev/null; then
    echo "pip not found. Installing pip..."
    sudo apt-get install -y python3-pip
else
    echo "pip is already installed."
fi

# Install mkdocs
echo "Installing dependencies [mkdocs], [tableauhyperapi]..."
pip install mkdocs
pip install tableauhyperapi

# Verify installation
echo "Verifying mkdocs installation..."
if python3 -c "import mkdocs" &> /dev/null; then
    echo "mkdocs successfully installed."
else
    echo "Failed to install mkdocs."
fi

echo "Verifying tableauhyperapi installation..."
if python3 -c "import tableauhyperapi" &> /dev/null; then
    echo "tableauhyperapi successfully installed."
else
    echo "Failed to install tableauhyperapi."
fi