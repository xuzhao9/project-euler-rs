#!/usr/bin/env bash
PROB_NO=$1
touch src/p$PROB_NO.rs
sed -i 's/p[0-9]*/p'"$PROB_NO"'/' src/main.rs
