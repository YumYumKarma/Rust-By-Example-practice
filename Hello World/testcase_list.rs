//handling sequential elements using ? operator

use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List{
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result
    {
        //Extract value using tuple indexing and create a refrence to 'vec'.
        let vec = &self.0;

        write!(f,"[")?;

        //Iterate over 'v' in 'vec' while enumerating the iteration count in 'count'
        for(count, v) in vec.iter().enumerate()
        {
            //For every elemnt except the first add ','
            //Use the ? operator to return on errors 
            if count!=0 {write!(f,", ")?;}
            write!(f,"{}:{}",count,v)?;
        }

        write!(f,"]")
    }
}

fn main()
{
    let v = List(vec!(9,12,22,69,11));
    println!("{}",v);
}

