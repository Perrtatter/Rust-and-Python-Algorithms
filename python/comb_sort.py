def swap_list(elements, x, y):
    temp = elements[x]
    elements[x] = elements[y]
    elements[y] = temp

def comb_sort(elements):
    between = len(elements)
    swap = False
    
    while between > 1 or swap == True:
        between = int(between / 1.3)
        
        if between < 1:
            between = 1
        
        i = 0
        swap = False
        
        while i < len(elements) - between:
            if elements[i] > elements[i + between]:
                swap_list(elements, i, i + between)
                swap = True
            
            i = i + 1
        

    return elements; 



elements = [3,5,4,2,1]
    
sorted_elements = comb_sort(elements)
print(sorted_elements)
