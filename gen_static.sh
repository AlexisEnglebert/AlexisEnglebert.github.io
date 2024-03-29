#!/bin/sh
set -xe

ROUTES=$(cat <<-END
/
END
)

rm -rf static
mkdir -p static
cp -r public static/public

cargo run &
sleep 5 # XXX should be enough, lol

for route in $ROUTES; do
	mkdir -p static$route
	curl http://localhost:8000$route -o static$route/index.html
done

kill $(jobs -p)