#!/usr/bin/bash

current-version() {
  grep -e '^version = ' "${1}" | grep -o "[0-9]\+\.[0-9]\+\.[0-9]\+"
}

current_todoistctl=$(current-version bin/todoistctl/Cargo.toml)
current_libtodoist=$(current-version lib/libtodoist/Cargo.toml)


version_in_lock=$( \
  grep -A 1 'name = "libtodoist"' Cargo.lock \
    | grep -e '^version =' \
    | grep -o "[0-9]\+\.[0-9]\+\.[0-9]\+")

echo " - Current todoistctl version:          ${current_todoistctl}"
echo " - Current libtodoist version:          ${current_libtodoist}"
echo " - libtodoist version in project file:  ${version_in_lock}"

echo ""
echo "Does this look good to you?"
echo ""
echo "Press ^C to exit or any key to continue..."
read

if [[ "${1}" = "" ]]; then
  cargo publish --package libtodoist
  cargo publish --package todoistctl
else
  cargo publish --registry "${1}" --package libtodoist
  cargo publish --registry "${1}" --package todoistctl
fi
