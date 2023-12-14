#!/bin/bash
#bash script that parses int from the post_number file and increments it, then copies the n-post directory into the posts directory while replacing instance of n with the incremented int 

if [ -z "$1" ]
  then
    echo "No title supplied"
    exit 0
fi

old=$(cat post_number.txt)
new=$(expr $old + 1)
echo $new
sed -i '' "s/${old}/${new}/" post_number.txt
cp -r post_template/n-post posts/
mv posts/n-post posts/$new-post
sed -i '' "s/&n/${new}/g" posts/$new-post/post_frontmatter.toml
date=$(date +%F)
echo $date
sed -i '' "s/&date/${date}/" posts/$new-post/post_frontmatter.toml
sed -i '' "s/&title/${1}/" posts/$new-post/post_frontmatter.toml

