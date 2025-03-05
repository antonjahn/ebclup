# For each subfolder, create a tar.gz file with the same name as the folder
# and the contents of the folder inside it. Strip the folder name.
# Usage: ./pack.sh

for dir in */; do
    dir=${dir%*/}
    tar -czf "${dir}.tar.gz" -C "${dir%*/}" .
done