# replace the text 255325274555.dkr.ecr.ap-south-1.amazonaws.com with the argument passed to the script
# example usage: ./update-repo-name.sh my-enterprise-repo
if [ -z "$1" ]; then
  echo "Usage: $0 361319394268.dkr.ecr.ap-south-1.amazonaws.com"
  exit 1
fi

find . -type f -exec sed -i '' "s/255325274555.dkr.ecr.ap-south-1.amazonaws.com/$1/g" {} +