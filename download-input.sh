#!/usr/bin/env bash

# Script that gets all available inputs not downloaded so far

BASE_LINK="https://adventofcode.com/2023/day"
RES_DIR=$PWD/res
COOKIE_FILE=$RES_DIR/cookie.txt

download_input() {
	for day in {01..30}
	do
		local link_day=$(echo $day | egrep -o '[1-9]+')
		local output_file="$RES_DIR/input_${day}.txt"
		if [ ! -f "$output_file" ]; then
			local link="$BASE_LINK/$link_day/input"
			echo "Downloading $link..."
			curl "$link" -b "$(cat $COOKIE_FILE)" --fail 1>$output_file 2>/dev/null
			# break if curl can't retrieve url (404)
			[ $? -eq 22 ] && echo "Can't access link! Stopping..." && rm $output_file && break
			echo "Saved to $RES_DIR/input_${day}.txt..."
		fi
	done
}

[ ! -d $RES_DIR ] && mkdir $RES_DIR
[ "$1" == "clean" ] && rm $RES_DIR/input*

download_input

