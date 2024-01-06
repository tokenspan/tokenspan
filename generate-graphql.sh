#!/bin/shell

# Set the root directory to search for .graphql files
root_directory="tokenspan-api/tests/graphql"

output_file="tokenspan-api/tests/graphql/generated.rs"

# List all files with the ".graphql" extension in the folder
graphql_files=$(find "$root_directory" -maxdepth 2 -type f -name "*.graphql")


if [ -e "$output_file" ]; then
    rm "$output_file"
    echo "Deleted existing $output_file"
fi

cat <<EOL >> "$output_file"
#![allow(dead_code)]
#![allow(unused_imports)]

EOL

# Loop over the found files
for file in $graphql_files; do
    echo "Processing file: $file"
    graphql-client generate -s schema.graphql -p 'crate::graphql' -O 'Debug,PartialEq' -o "$root_directory/generated" "$file" > /dev/null 2>&1

    # Replace both substrings with an empty string using sed
    filename=$(echo "$file" | sed -E 's|tokenspan-api/tests/graphql/.*/||;s|\.graphql$||')

    echo "mod $filename;" >> "$output_file"
    echo "pub use $filename::*;" >> "$output_file"
done

cat <<EOL >> "$output_file"

pub type UUID = uuid::Uuid;
pub type NaiveDateTime = chrono::NaiveDateTime;
pub type JSON = serde_json::Value;
pub type Cursor = String;
EOL