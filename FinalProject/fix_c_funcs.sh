#!/bin/sh
mkdir build
cd build
cmake ..
make 
mv ./c_funcs*.so ../finalproject
cd ..
python3 ./run.py