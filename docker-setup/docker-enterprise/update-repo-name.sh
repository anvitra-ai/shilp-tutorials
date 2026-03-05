# replace the text <replace-with-enterprise-repo> with the argument passed to the script
# example usage: ./update-repo-name.sh my-enterprise-repo
if [ -z "$1" ]; then
  echo "Usage: $0 361319394268.dkr.ecr.ap-south-1.amazonaws.com"
  exit 1
fi

# find all the occurence of <replace-with-enterprise-repo> in docker-compose file and replace it with the argument passed to the script
sed -i '' "s/<replace-with-enterprise-repo>/$1/g" docker-compose.yml