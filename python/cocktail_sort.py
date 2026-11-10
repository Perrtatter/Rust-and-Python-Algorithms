def swap_list(elements, x, y):
    temp = elements[x]
    elements[x] = elements[y]
    elements[y] = temp

def cocktail_sort(elements):
    swap = True
  
    while swap:
        swap = False
        
        for i in range(len(elements) -1):
            if elements[i] > elements[i + 1]:
                swap_list(elements, i, i + 1)
                swap = True
        
        
        
        for i in range(len(elements) - 2, -1, -1):
            if elements[i] > elements[i + 1]:
                swap_list(elements, i, i + 1)
                swap = True
            
        
    
    
    return elements; 



elements = [3,5,4,2,1]
    
sorted_elements = cocktail_sort(elements)
print(sorted_elements)
