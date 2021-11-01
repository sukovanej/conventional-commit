#!/bin/sh

printf "\n\n=============================================\n\n"
echo "Seting up the git alias..."

git config --global alias.cc 1> /dev/null 2>&1

COMMAND_ALREADY_SET=$?

[ $COMMAND_ALREADY_SET -eq 1 ] && git config --global alias.cc conventional-commit && echo "Done, git alias set!" && exit 0

printf "Git alias 'cc' is already set, rewrite? [y/N]: "
read ANSWER

[ $ANSWER = "y" ] && git config --global alias.cc conventional-commit && echo "Done, git alias set!" && exit 0

echo "Done without git alias"
