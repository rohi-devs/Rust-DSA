fn main(){
    let mut k = vec![1,2,4,44,32,5,6];
    bubble_sort(&mut k);
    println!("{:?}", &k);
}

fn bubble_sort(arr:&mut Vec<u8>){

    let len = arr.len();
    let mut swapped;

    loop {
        swapped  = false;

        for i in 0..len-1{
            if arr[i] > arr[i+1] {
                arr.swap(i, i+1);
                swapped=true;
            }
        }
        
        if !swapped{
            break;
        }
    }
}
