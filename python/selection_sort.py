def selection_sort(elements):

    for i in range(len(elements)):
        min = i

        for j in range(i+1, len(elements)):

            if elements[min] > elements[j]:
                min = j


        tmp = elements[i]
        elements[i] = elements[min]
        elements[min] = tmp
        
    return elements


elements = [3,5,2,6,1]

sorted_elements= selection_sort(elements)
print(sorted_elements)
