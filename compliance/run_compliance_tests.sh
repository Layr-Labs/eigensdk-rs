#!/usr/bin/env bash

# Read every line of the funtion_list.txt file
while read line; do
    # If the line is not empty
    if [ -n "$line" ]; then
        # Print the line
        echo "$line"
        # Run the function tests with that name
        #`cargo t --test $line -- --nocapture`
        cargo test -p eigen-services-blsaggregation "$line" -- --nocapture
        # TODO!!! $line
    fi
done < function_list.txt
