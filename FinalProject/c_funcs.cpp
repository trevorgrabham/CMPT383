#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include <cmath>
#include <iostream>

namespace py = pybind11;


int sum(py::list lst){
	int sum=0;
	for(int i=0;i<lst.size();i++){
		sum += lst[i].cast<int>();
	}
	return sum;
}

double avg(py::list lst){
	return double(sum(lst))/lst.size();
}

int* max(py::list lst){
	int max = lst[0].cast<int>();
	int index = 0;
	for(int i=1;i<lst.size();i++){
		if(lst[i].cast<int>() > max){
			max = lst[i].cast<int>();
			index = i;
		}
	}
	int* res = new int[2];
	res[0] = max;
	res[1] = index;
	return res;
}

int* min(py::list lst){
	int min = lst[0].cast<int>();
	int index = 0;
	for(int i=1;i<lst.size();i++){
		if(lst[i].cast<int>() < min){
			min = lst[i].cast<int>();
			index = i;
		}
	}
	int* res = new int[2];
	res[0] = min;
	res[1] = index;
	return res;
}

double std_dev(py::list lst){
	double mean = avg(lst);
	double sqr_diff = 0;
	for(int i=0;i<lst.size();i++){
		sqr_diff += std::pow(lst[i].cast<int>() - mean,2);
	}
	return pow(sqr_diff/lst.size(),0.5);
}

// based upon the average of whatever values are sent in (weight or reps)
int best_pos(py::list lst, py::list pos){
	double sum_of_pos[20];
	int count_of_pos[20];
	int position = 0;
	for(int i=0;i<20;i++){
		sum_of_pos[i] = 0;
		count_of_pos[i] = 0;
	}
	for(int i=0;i<lst.size();i++){
		sum_of_pos[pos[i].cast<int>()-1] += lst[i].cast<int>();
		count_of_pos[pos[i].cast<int>()-1]++;
	}
	
	for(int i=0;i<20;i++){
		if(count_of_pos[i] != 0){
			sum_of_pos[i] /= count_of_pos[i];
		}
	}
	
	for(int i=1;i<20;i++){
		if(sum_of_pos[i] > sum_of_pos[position]){
			position = i;
		}
	}
	return position+1;
}

int consistant_pos(py::list lst, py::list pos){
	double sum_of_pos[20];
	int count_of_pos[20];
	double sqr_diff_of_pos[20];
	int position = 0;
	for(int i=0;i<20;i++){
		sum_of_pos[i] = 0;
		count_of_pos[i] = 0;
		sqr_diff_of_pos[i] = 0;
	}
	for(int i=0;i<lst.size();i++){
		sum_of_pos[pos[i].cast<int>()-1] += lst[i].cast<int>();
		count_of_pos[pos[i].cast<int>()-1]++;
	}
	std::cout << "Sum: ";
	for (int i=0;i<lst.size();i++){
		std::cout << sum_of_pos[i] << " ";
	}
	std::cout << std::endl;
	std::cout << "Count: ";
	for (int i=0;i<lst.size();i++){
		std::cout << count_of_pos[i] << " ";
	}
	for(int i=0;i<20;i++){
		if(count_of_pos[i] != 0){
			sum_of_pos[i] /= count_of_pos[i];
		}
	}
	std::cout << "Avg: ";
	for (int i=0;i<lst.size();i++){
		std::cout << sum_of_pos[i] << " ";
	}
	std::cout << std::endl;
	// sum_of_pos now is the avg of pos
	for(int i=0;i<lst.size();i++){
		sqr_diff_of_pos[pos[i].cast<int>()-1] += std::pow(lst[i].cast<int>() - sum_of_pos[pos[i].cast<int>()-1],2);
	}
	std::cout << "SqrDiff: ";
	for (int i=0;i<lst.size();i++){
		std::cout << sqr_diff_of_pos[i] << " ";
	}
	std::cout << std::endl;
	for(int i=0;i<20;i++){
		if(count_of_pos[i] != 0){
			sqr_diff_of_pos[i] /= count_of_pos[i];
		}
	}
	std::cout << "SqrDiffAvg: ";
	for (int i=0;i<lst.size();i++){
		std::cout << sqr_diff_of_pos[i] << " ";
	}
	std::cout << std::endl;
	for(int i=1;i<20;i++){
		if(sqr_diff_of_pos[i] < sqr_diff_of_pos[position] && count_of_pos[i] != 0){
			position = i;
		}
	}
	return position+1;
}

int total_lifted(py::list weight, py::list reps){
	int total = 0;
	for(int i=0;i<weight.size();i++){
		total += weight[i].cast<int>() * reps[i].cast<int>();
	}
	return total;
}



PYBIND11_MODULE(c_funcs, handle){
	handle.doc() = "functions written in C++ for optimization";
	handle.def("sum", &sum);
	handle.def("avg", &avg);
	handle.def("max", &max);
	handle.def("min", &min);
	handle.def("std_dev", &std_dev);
	handle.def("best_pos", &best_pos);
	handle.def("consistant_pos", &consistant_pos);
	handle.def("total_lifted", &total_lifted);
}
