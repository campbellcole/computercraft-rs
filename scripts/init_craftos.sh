#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
CC_RS_DIR="$( dirname "$SCRIPT_DIR" )"
LUA_DIR="$CC_RS_DIR/lua"

CRAFTOS_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/craftos-pc"

if [ ! -d "$CRAFTOS_DIR" ]; then
    echo "Could not locate CraftOS-PC installation at $CRAFTOS_DIR"
    echo "Please install CraftOS-PC and try again."
    exit 1
fi

print_usage() {
    echo "Usage: $0 <computer-id> [--link] [--config]"
    echo -e "  <computer-id>: The ID of the computer to initialize"
    echo -e "   --link,   -l: Link the worker files instead of copying them"
    echo -e "   --config, -c: Also copy/link the 'example_config.json' file"
    echo -e "   --help,   -h: Print this help message"
}

MODE="copy"
USE_EXAMPLE_CONFIG=0

while [[ $# -gt 0 ]]; do
    case $1 in
        -l|--link)
            MODE="link"
            shift
            ;;
        -c|--config)
            USE_EXAMPLE_CONFIG=1
            shift
            ;;
        -h|--help)
            print_usage
            exit 0
            shift
            ;;
        *)
            COMPUTER_ID="$1"
            shift
            ;;
    esac
done

if [ -z "$COMPUTER_ID" ]; then
    print_usage
    exit 1
fi

COMPUTER_DIR="$CRAFTOS_DIR/computer/$COMPUTER_ID"

if [ ! -d "$COMPUTER_DIR" ]; then
    echo "Could not locate computer '$COMPUTER_ID' in '$CRAFTOS_DIR/computer'."
    echo "Please create the computer and try again."
    exit 1
fi

ESCAPE_ROPE="$(pwd)"
# geronimo
cd "$COMPUTER_DIR"

FILES=("worker.lua" "worker")

if [ "$USE_EXAMPLE_CONFIG" == "1" ]; then
    FILES+=("example_config.json")
fi

for file in "${FILES[@]}"; do
    FILE_TYPE="file"
    if [ -d "$file" ]; then
        FILE_TYPE="directory"
    fi
    echo "$FILE_TYPE: $file"
    if [ -e "$file" ]; then
        rm "$file"
        echo "  -> previous $FILE_TYPE removed"
    fi
    if [ "$MODE" == "copy" ]; then
        cp "$LUA_DIR/$file" "$file"
        echo "  -> copied"
    else
        ln -s "$LUA_DIR/$file" "$file"
        echo "  -> linked"
    fi
done

cd "$ESCAPE_ROPE"