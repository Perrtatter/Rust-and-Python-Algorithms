def bubble_sort(elements):

    for i in range(len(elements)):

        for j in range(0, len(elements)-i-1):

            if elements[j] > elements[j+1]:
                elements[j], elements[j+1] = elements[j+1], elements[j]

    return elements


elements = [3,5,2,6,1]

sorted_elements= bubble_sort(elements)
print(sorted_elements)
