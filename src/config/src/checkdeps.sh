#!/usr/bin/env bash

REPO_ROOT="$(cd "$(dirname "$(realpath "${BASH_SOURCE[0]}")")" &>/dev/null && cd ../../.. && pwd)"
export REPO_ROOT
echo "Repo: $REPO_ROOT"
SRC_DIR="$REPO_ROOT/src"

set -eax

FAILED=0

# we can assume sh is installed or else we wouldn't be here

XINERAMA=${XINERAMA:-true}
CC=${CC:-cc}
printf "Checking for '%s'... " "$CC"

if command -v "$CC" >/dev/null 2>&1; then
    echo "yes"
else
    echo "no"
    echo >&2 "Build-time dependency $CC not installed. Install it or change the C compiler used in config.mk"
    FAILED=1
fi

printf "Checking for X11 headers... "
if $CC -c "$SRC_DIR/headers/src/xlib.h" -o tmp.gch; then
    rm tmp.gch
    echo "yes"
else
    echo "no"
    echo >&2 "Build-time dependency <X11/Xlib.h> is not present. Install the xorg development packages"
    rm -f tmp.gch
    FAILED=1
fi

printf "Checking for fontconfig headers... "
if $CC -c "$SRC_DIR/headers/src/fontconfig.h" -o tmp.gch; then
    rm tmp.gch
    echo "yes"
else
    echo "no"
    echo >&2 "Build-time dependency <fontconfig/fontconfig.h> is not present. Install fontconfig packages"
    rm -f tmp.gch
    FAILED=1
fi

if [ "$XINERAMA" = "true" ]; then
    printf "Checking for xinerama headers... "
    if $CC -c "$SRC_DIR/headers/src/xinerama.h" -o tmp.gch; then
        rm tmp.gch
        echo "yes"
    else
        echo "no"
        echo >&2 "Build-time dependency <extensions/Xinerama.h> is not present. Install xinerama package(s) or disable the feature in config.mk"
        rm -f tmp.gch
        FAILED=1
    fi
fi

if [ $FAILED != 0 ]; then
    exit 1
fi
