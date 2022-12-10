#!/bin/bash
#check if day arg is passed
if [ $# -eq 0 ]
  then
    echo "No argument for day num supplied"
    exit 1
fi

echo "Creating day $1"

#goto aoc dir
cd "${0%/*}"
cd ..

#make new dir
mkdir -p "day-$1"

#copy template
cp -r template/template_files/. "day-$1"

#replace day num
cd "day-$1"

#replace day num in files
if [ "$(uname -s)" == "Darwin" ]; then
    echo "using darwin sed for day replacement"
    sed -i '' -e "s/DAY_NUM/$1/g" Cargo.toml 
    sed -i '' -e "s/DAY_NUM/$1/g" src/bin/part1.rs
    sed -i '' -e "s/DAY_NUM/$1/g" src/bin/part2.rs
else
    echo "using linux sed for day replacement"
    sed -i -e "s/DAY_NUM/$1/g" Cargo.toml 
    sed -i -e "s/DAY_NUM/$1/g" src/bin/part1.rs
    sed -i -e "s/DAY_NUM/$1/g" src/bin/part2.rs
fi
