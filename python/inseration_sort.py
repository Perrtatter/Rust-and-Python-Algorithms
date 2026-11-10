def swap(elements, x, y):
    temp = elements[x]
    elements[x] = elements[y]
    elements[y] = temp


def inseration_sort(elements):
    i = 1
    while i < len(elements):
        j = i

        while j > 0 and elements[j-1] > elements[j]:
            swap(elements, j, j-1)
            j = j - 1

        i +=1

    return elements


elements = [3, 5, 2, 6, 1]
sorted_elements = inseration_sort(elements)

print(sorted_elements)