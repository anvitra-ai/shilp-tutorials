# replace the text <replace-with-enterprise-repos> with the argument passed to the script
# example usage: ./update-repo-name.sh my-enterprise-repo
if [ -z "$1" ]; then
  echo "Usage: $0 <replace-with-enterprise-repos>"
  exit 1
fi

find . -type f -exec sed -i '' "s/<replace-with-enterprise-repos>/$1/g" {} +