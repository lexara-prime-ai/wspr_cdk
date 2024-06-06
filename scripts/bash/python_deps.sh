#!/bin/bash

# Check if pip is installed.
if ! command -v pip &>/dev/null; then
	echo "pip not found. Installing pip..."
	sudo apt-get install -y python3-pip
else
	echo "pip is already installed."
fi


# Modules that will be installed/upgraded.
modules=("mkdocs" "maturin" "patchelf" "tableauhyperapi" "google-api-python-client" "google-auth-httplib2" "google-auth-oauthlib")

echo "Installing dependencies: ${modules[*]}..."
pip install "${modules[@]}" --upgrade

# Verify module installation.
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

# The following dictionary contains module to import name mappings.
declare -A module_import_map=(
	["mkdocs"]="mkdocs"
	["maturin"]="maturin"
	["patchelf"]="patchelf"
	["tableauhyperapi"]="tableauhyperapi"
	["google-api-python-client"]="googleapiclient"
	["google-auth-httplib2"]="google_auth_httplib2"
	["google-auth-oauthlib"]="google_auth_oauthlib"
)

# Verify installation of each module.
for module in "${!module_import_map[@]}"; do
	verify_installation "${module}" "${module_import_map[${module}]}"
done

# The [modules] array contains the names of all the modules to be installed.
# The [dictionary] [module_import_map] maps module names to their respective import names.
# The verify_installation function takes a module name and its import name as arguments, checks if the module can be imported, and prints the appropriate message.
# The script iterates over the module_import_map dictionary to verify the installation of each module using the verify_installation function.