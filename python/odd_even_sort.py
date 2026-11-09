def swap(elements, x, y):
    temp = elements[x]
    elements[x] = elements[y]
    elements[y] = temp
    

def odd_even_sort(elements):
    sorted_flag = False

    while not sorted_flag:
        sorted_flag = True

        for i in range(1, len(elements) - 1, 2):

            if elements[i] > elements[i + 1]:
                swap(elements, i, i + 1)
                sorted_flag = False


        for j in range(0, len(elements) - 1, 2):

            if elements[j] > elements[j + 1]:
                swap(elements, j, j + 1)
                sorted_flag = False

                
    return elements


elements = [3, 5, 2, 6, 1]
sorted_elements = odd_even_sort(elements)
print(sorted_elements)