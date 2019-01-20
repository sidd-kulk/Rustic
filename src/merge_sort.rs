fn merge_sort(vec: &Vec<i16>, output: &mut Vec<i16>) {
    // println!("Input = {:?}", vec);
    
    if(vec.len() <2 ){
        return;
    }
    let mid_size = vec.len()/2;
    
    // println!("{}", mid_size);
    
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
    // println!("Output : {:?}", output);
}

fn merge(left: &Vec<i16>, right: &Vec<i16>, output: &mut Vec<i16>) {
        println!("Left = {:?}, Right = {:?}", left, right);
        let mut k = 0;
        let n_left = left.len();   
        let n_right = right.len();
        let (mut i, mut j) = (0, 0);
        while(i< n_left && j< n_right){
            // println!("Left: {}, Right: {}", left[i], right[j]);
            if(left[i] <= right[j]){
                println!("1111");
                output.push(left[i]);
                i += 1
            } else {
                println!("2222");
                output.push(right[j]);
                j += 1;
            }
            k += 1;
        }
        
        while(i < n_left){
            println!("3333");
            output.push(left[i]);
            i += 1;
            k += 1;
        }
        
        while(j < n_right){
            println!("4444");
            output.push(right[j]);
            j += 1;
            k += 1;
        }
        println!("End of merge:: {:?}", output);
        
}
