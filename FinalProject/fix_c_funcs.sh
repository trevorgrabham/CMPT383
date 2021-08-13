#!/bin/sh
mkdir build
cd build
cmake ..
make 
mv ./c_funcs*.so ../finalproject
cd ..
rm -r ./build
python3 ./run.py