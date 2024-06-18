#!/bin/bash

# Terminal Colors.
NORMAL=$(tput sgr0)
UNDERLINE=$(tput smul)
RED=$(tput setaf 1)
BLUE=$(tput setaf 4)
GREEN=$(tput setaf 2)
# BLACK=$(tput setaf 0)
# YELLOW=$(tput setaf 3)
# LIME_YELLOW=$(tput setaf 190)
# POWDER_BLUE=$(tput setaf 153)
# MAGENTA=$(tput setaf 5)
# CYAN=$(tput setaf 6)
# WHITE=$(tput setaf 7)
# BRIGHT=$(tput bold)
# NORMAL=$(tput sgr0)
# REVERSE=$(tput smso)
# BLINK=$(tput blink)

# Install [patchelf]
sudo apt-get -y install patchelf -y

# Check if pip is installed.
if ! command -v pip &>/dev/null; then
	echo "${BLUE}pip not found. Installing pip...${NORMAL}"
	sudo apt-get install -y python3-pip
else
	echo "${GREEN}pip is already installed.${NORMAL}"
fi

# Modules that will be installed/upgraded.
modules=("modal" "mkdocs" "maturin" "patchelf" "tableauhyperapi" "google-api-python-client" "google-auth-httplib2" "google-auth-oauthlib")

echo "${BLUE}Installing dependencies: ${modules[*]}...${NORMAL}"
pip install "${modules[@]}" --upgrade

# Verify module installation.
verify_installation() {
	local module=$1
	local import_name=$2
	printf "\nVERIFYING %s INSTALLATION...\n" "${module}"
	if python3 -c "import ${import_name}" &>/dev/null; then
		printf "%s ${UNDERLINE}${GREEN}SUCCESSFULLY${NORMAL} INSTALLED.\n\n" "${module}"
	else
		echo "${UNDERLINE}${RED}FAILED${NORMAL} TO INSTALL ${module}."
	fi
}

# The following dictionary contains module to import name mappings.
declare -A module_import_map=(
	["modal"]="modal" # To Do -> Update Docker configuration to include modal cli configuration.
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
