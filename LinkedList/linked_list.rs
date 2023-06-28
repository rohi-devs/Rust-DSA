#[derive(Debug,Clone)]
enum Address{
    Address(Box<LinkedList>),
    Nil
}
#[derive(Debug,Clone)]
pub struct LinkedList{
    val : Option<u32>,
    next : Address,
}
impl LinkedList{
    pub fn new()->Self{
        Self{
            val:None,
            next:Address::Nil,
        }
    }
    pub fn append(&mut self,value:u32){
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
    pub fn delete(&mut self, value:u32){
        match self.next {
            Address::Address(ref mut next_address)=>{
                match next_address.val {
                    Some(x)=>{
                        if x==value {
                            println!("Deleting Value {}",x);
                            self.next = next_address.next.clone();
                        }
                        else {
                            next_address.delete(value);
                        }
                    },
                    // No operation 
                    None => todo!(),
                }
            }
            Address::Nil => {
                match self.val {
                    Some(x) => {
                        if x==value {
                            self.val = None;
                        }
                        else {
                            println!("The element --> {} <--not found",value);
                        }
                    },
                    // No operation
                    None => todo!(),
                }
            }
        }
    }
    pub fn list(&self){
        if self.val == None {
            println!("List is empty");
        }
        else {
            match self.val {
                Some(x)=> {
                    println!("{x}");
                }
                // No operation
                None => todo!(),
            }
            match  self.next{
                Address::Address(ref next_address)=>next_address.list(),
                Address::Nil=>{}
            }
        }
    }
}
