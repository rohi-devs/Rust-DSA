fn main() {
    println!("Hello, world!");
    let veto:Vec<u32> = vec![1,4,5,24,36,38,41,63,455,3566];
    let result = bin_serch(&veto,414);
    println!("{:?}", result);
}
fn bin_serch(arr:&Vec<u32>,target:u32)->i32{
    let mut l = 0;
    let mut arr_length=arr.len()-1;
    while l<=arr_length {
        let  mid:usize = arr_length-l/2;
        if arr[mid] == target{ 
            return mid as i32
        }
        if arr[mid] < target{
            l=mid+1;
        }
        else {
            arr_length = mid-1;
        }
    }
     -1
}
