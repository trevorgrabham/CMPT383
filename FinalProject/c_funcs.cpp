#include <pybind11/pybind11.h>
#include <pybind11/stl.h>

namespace py = pybind11;

int add(int x, int y) {
	return x + y;
}

int sub(int x, int y) {
	return x - y;
}

PYBIND11_MODULE(c_funcs, handle){
	handle.doc() = "functions written in C for optimization";
	handle.def("add", &add);
	handle.def("sub", &sub);
}
