#include <pybind11/pybind11.h>
#include <pybind11/stl.h>

namespace py = pybind11;

const MAX_SETS_IN_WORKOUT = 20;
const INT_MAX = 2147483647;

int sum(int* lst, int len){
	int sum=0;
	for(int i=0;i<len;i++){
		sum += lst[i];
	}
	return sum;
}

double avg(int* lst, int len){
	return double(sum(lst, len))/count;
}

int* max(int* lst, int len){
	int max = lst[0];
	int index = 0;
	for(int i=1;i<len;i++){
		if(lst[i] > max){
			max = lst[i];
			index = i;
		}
	}
	res = new Int[2];
	res[0] = max;
	res[1] = index;
	return res;
}

int* min(int* lst), int len{
	int min = lst[0];
	int index = 0;
	for(int i=1;i<len;i++){
		if(lst[i] < min){
			min = lst[i];
			index = i;
		}
	}
	res = new Int[2];
	res[0] = min;
	res[1] = index;
	return res;
}

double std_dev(int* lst, int len){
	double mean = avg(lst, len);
	double sqr_diff = 0;
	for(int i=0;i<len;i++){
		sqr_diff += (lst[i] - mean)^2;
	}
	return (sqr_diff/len)^(0.5);
}

// based upon the average of whatever values are sent in (weight or reps)
int best_pos(int* lst, int* pos, int len){
	int* sum_of_pos = Int[MAX_SETS_IN_WORKOUT];
	int* count_of_pos = Int[MAX_SETS_IN_WORKOUT];
	int position = 0;
	for(int i=0;i<MAX_SETS_IN_WORKOUT;i++){
		sum_of_pos[i] = 0;
		count_of_pos[i] = 0;
	}
	for(int i=0;i<len;i++){
		sum_of_pos[pos[i]] += lst[i];
		count_of_pos[pos[i]]++;
	}
	for(int i=0;i<MAX_SETS_IN_WORKOUT;i++){
		if(count_of_pos[i] != 0){
			sum_of_pos[i] /= count_of_pos[i];
		}
	}
	for(int i=1;i<MAX_SETS_IN_WORKOUT;i++){
		if(sum_of_pos[i] > sum_of_pos[position]){
			position = i;
		}
	}
	return position;
}

int consistant_pos(int* lst, int len){
	int* sum_of_pos = Int[MAX_SETS_IN_WORKOUT];
	int* count_of_pos = Int[MAX_SETS_IN_WORKOUT];
	int* sqr_diff_of_pos = Int[MAX_SETS_IN_WORKOUT];
	int position = 0;
	for(int i=0;i<MAX_SETS_IN_WORKOUT;i++){
		sum_of_pos[i] = 0;
		count_of_pos[i] = 0;
		sqr_diff_of_pos[i] = 0;
	}
	for(int i=0;i<len;i++){
		sum_of_pos[pos[i]] += lst[i];
		count_of_pos[pos[i]]++;
	}
	for(int i=0;i<MAX_SETS_IN_WORKOUT;i++){
		if(count_of_pos[i] != 0){
			sum_of_pos[i] /= count_of_pos[i];
		}
	}
	// sum_of_pos now is the avg of pos
	for(int i=0;i<len;i++){
		sqr_diff_of_pos[pos[i]] += (lst[i] - sum_of_pos[pos[i]])^2;
	}
	for(int i=0;i<MAX_SETS_IN_WORKOUT;i++){
		if(count_of_pos[i] != 0){
			sqr_diff_of_pos[i] /= count_of_pos[i];
		}
	}
	for(int i=1;i<MAX_SETS_IN_WORKOUT;i++){
		if(sqr_diff_of_pos[i] < sqr_diff_of_pos[position]){
			position = i;
		}
	}
return position;
}

int total_lifted(int* weight, int* reps, int len){
	int total = 0;
	for(int i=0;i<len;i++){
		total += weight[i] * reps[i];
	}
	return total;
}



PYBIND11_MODULE(c_funcs, handle){
	handle.doc() = "functions written in C++ for optimization";
	handle.def("sum", &sum);
	handle.def("total", &total);
	handle.def("avg", &avg);
	handle.def("max", &max);
	handle.def("min", &min);
	handle.def("std_dev", &std_dev);
	handle.def("best_pos", &best_pos);
	handle.def("consistant_pos", &consistant_pos);
	handle.def("total_lifted", &total_lifted);
}
