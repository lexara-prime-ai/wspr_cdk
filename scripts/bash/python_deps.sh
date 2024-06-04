#!/bin/bash

# Check if pip is installed
if ! command -v pip &>/dev/null; then
	echo "pip not found. Installing pip..."
	sudo apt-get install -y python3-pip
else
	echo "pip is already installed."
fi

# List of modules to install
modules=("mkdocs" "tableauhyperapi" "google-api-python-client" "google-auth-httplib2" "google-auth-oauthlib")

# Install modules
echo "Installing dependencies: ${modules[*]}..."
pip install "${modules[@]}" --upgrade

# Function to verify module installation
verify_installation() {
	local module=$1
	local import_name=$2
	echo "Verifying ${module} installation..."
	if python3 -c "import ${import_name}" &>/dev/null; then
		echo "${module} successfully installed."
	else
		echo "Failed to install ${module}."
	fi
}

# Dictionary of module to import name mappings
declare -A module_import_map=(
	["mkdocs"]="mkdocs"
	["tableauhyperapi"]="tableauhyperapi"
	["google-api-python-client"]="googleapiclient"
	["google-auth-httplib2"]="google_auth_httplib2"
	["google-auth-oauthlib"]="google_auth_oauthlib"
)

# Verify installation of each module
for module in "${!module_import_map[@]}"; do
	verify_installation "${module}" "${module_import_map[${module}]}"
done
