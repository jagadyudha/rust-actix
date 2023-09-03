#!/bin/bash

# Get the current date and time in the desired format (YYYYMMDDHHMMSS)
current_datetime=$(date "+%Y%m%d%H%M%S")

# Define the dynamic string
dynamic_string="rename_this"

# Create the folder with the format
folder_name="${current_datetime}_${dynamic_string}"

# Make the directory
mkdir "migrations/$folder_name"

echo "Created folder: $folder_name"