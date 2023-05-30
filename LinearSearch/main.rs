fn main() {
    println!("{:?}", "Hello World");
    let man:Vec<i32> = vec!{1,2,3,4,5};
    let k= linear_search(man,&10);
    let result = match k{
        Ok(i)=>i as isize,
        Err(j)=>j as isize
    };
    println!("{:?}",result);
}

fn linear_search(arr:Vec<i32>,target:&i32)->Result<usize,isize>{
    for (i,j) in arr.iter().enumerate(){
        if j==target{
            return Ok(i);
        }
    };
    Err(-1)
}
