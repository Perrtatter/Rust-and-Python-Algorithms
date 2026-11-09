def dichto(elements,x):
    elements.sort()

    left = 0
    right = len(elements) -1

    while left <= right:
        center = int((right + left)/2)

        if elements[center] < x:
            left = center +1

        elif elements[center] > x:
            right = center -1

        else:
            return center
    return -1



elements = [2,4,8,7,9,15,12,1,17]

x = dichto(elements,15)
print(x)

# x = 7
# elements[7] = 15 

