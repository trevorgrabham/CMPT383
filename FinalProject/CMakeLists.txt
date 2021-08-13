# taken from https://www.youtube.com/watch?v=_5T70cAXDJ0
cmake_minimum_required(VERSION 3.4)
project(c_funcs)
add_subdirectory(pybind11)
pybind11_add_module(c_funcs c_funcs.cpp)
