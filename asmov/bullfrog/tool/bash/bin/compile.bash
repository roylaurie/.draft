#!/bin/bash
set -o errexit -o pipefail -o privileged -o nounset
cd "$(npm prefix)"

# compact the namespace configuration schema
jq -c < ./json/schema/package/modules.schema.json > ./json/schema/package/modules.schema.min.json

# compact the actual namespace configuration into modules.json
jq -c < ./json/data/package/modules.json > ./json/data/package/modules.min.json

# validate the minified namespace configuration against the minified schema
ajv -s ./json/schema/package/modules.schema.min.json -d ./json/data/package/modules.min.json

# convert json into the frogcfg bash script
./tool/js/json2bash.mjs \
    ./json/schema/package/modules.schema.min.json \
    ./json/data/package/modules.min.json \
    > ./bash/cfg/package/modules.cfg.bash

