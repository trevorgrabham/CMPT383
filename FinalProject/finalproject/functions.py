def average(lst):
    sum = 0
    total = 0
    for item in lst:
        sum += item
        total += 1
    return sum/total


def max(lst):
    max = -1
    for item in lst:
        if item > max:
            max = item
    return max

def min(lst):
    min = lst[0]
    for item in lst:
        if item < min:
            min = item
    return min

def std_dev(lst):
    mean = average(lst)
    sum = 0
    total = 0
    for item in lst:
        sum += pow(item-mean,2)
        total += 1
    return pow(sum/total,0.5)

if __name__ ==  '__main__':
    print(average([9,2,5,4,12,7,8,11,9,3,7,4,12,5,4,10,9,6,9,4]))
