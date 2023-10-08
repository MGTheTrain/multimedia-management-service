#!/bin/bash

cd ..
find . -name '*.rs' -exec rustfmt \;
cd -