fn main() {
    let mut a:Vec<u32> = vec!{32,23,123,2,423,3,2,1};
    a = mergersort(a.clone(), 0, a.len());
    println!("{:?}", a);
}

fn mergersort(mut arr:Vec<u32>,l:usize,r:usize)->Vec<u32>{
    if l < r-1 {
        let mid = (l+(r-l)/2);
        arr = mergersort(arr, l, mid);
        arr = mergersort(arr, mid, r);
        arr = merge(arr, l, mid, r);
    }
    arr
}

fn merge(mut arr:Vec<u32>,l:usize,mid:usize,r:usize)->Vec<u32>{
    let n1 = mid-l;
    let n2 = r-mid;
    let mut l1 = arr.clone();
    let mut l2 = arr.clone();
    let L = &l1[l..mid];
    let R = &l2[mid..r];
    let mut i =0;
    let mut j=0;
    let mut k =l;
    while i < n1 && j < n2 {
        if L[i] < R[j] {
            arr[k] = L[i];
            i=i+1;
        }else {
            arr[k] = R[j];
            j=j+1;
        }
        k=k+1;
    }
    while i < n1 {
        arr[k] = L[i];
        i=i+1;
        k=k+1;
    }
    while j < n2 {
        arr[k] = R[j];
        j=j+1;
        k=k+1;
    }

    println!("{:?}", arr);

    return arr;
}
