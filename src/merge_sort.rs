fn merge_sort(vec: &Vec<i16>, output: &mut Vec<i16>) {

    if(vec.len() <2 ){
        return;
    }
    let mid_size = vec.len()/2;
    
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();
    
    for i in 0..mid_size {
        left_vec.push(vec[i]);
    }
    
    for i in (mid_size)..vec.len(){
        right_vec.push(vec[i]);
    }
    
    println!("Left vector = {:?}", left_vec);
    println!("Rigth vector = {:?}", right_vec);
    
    merge_sort(&left_vec, output);
    merge_sort(&right_vec, output);
    
    merge(&left_vec, &right_vec, output);
}

fn merge(left: &Vec<i16>, right: &Vec<i16>, output: &mut Vec<i16>) {
        let mut k = 0;
        let n_left = left.len();   
        let n_right = right.len();
        let (mut i, mut j) = (0, 0);
        while(i< n_left && j< n_right){
            if(left[i] <= right[j]){
                output.push(left[i]);
                i += 1
            } else {
                output.push(right[j]);
                j += 1;
            }
            k += 1;
        }
        
        while(i < n_left){
            output.push(left[i]);
            i += 1;
            k += 1;
        }
        
        while(j < n_right){
            output.push(right[j]);
            j += 1;
            k += 1;
        }
        println!("End of merge:: {:?}", output);
        
}
