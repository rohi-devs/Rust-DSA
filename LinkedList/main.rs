// Only the append method is implemented
#[derive(Debug)]
enum Address{
    Address(Box<LinkedList>),
    Nil
}
#[derive(Debug)]
struct LinkedList{
    val : Option<u32>,
    next : Address,
}
impl LinkedList{
    fn new()->Self{
        Self{
            val:None,
            next:Address::Nil,
        }
    }
    fn append(&mut self,value:u32){
        if self.val == None {
            self.val = Some(value);
        }
        else {
            match self.next {
                Address::Address(ref mut next_address) => {
                    next_address.append(value);
                }
                Address::Nil =>{
                    let node = LinkedList{
                        val:Some(value),
                        next:Address::Nil,
                    };
                    self.next = Address::Address(Box::new(node));
                }
            }
        }
    }
    fn list(&self){
        if self.val == None {
            println!("List is empty");
        }
        else {
            match self.val {
                Some(x)=> {
                    println!("{x}");
                }
                None=>{
                  // No operation
                }
            }
            match  self.next{
                Address::Address(ref next_address)=>next_address.list(),
                Address::Nil=>{
                  // No operation
                }
            }
        }
    }
}
fn main() {
    let mut lst = LinkedList::new();
    lst.append(5);
    lst.append(2);
    lst.list();
    println!("{:?}",lst);
}
