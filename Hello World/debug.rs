//we will derive fmt::Debug implementation for the 'Structure'.

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a>
{
    name: &'a str,
    age: u8
}

fn main()
{
    //printing with {:?} for debug statements also structure doesnt implement fmt::Display
    //printing with {:?} is similar to printing with {}.
    println!("{:?} months in a year.",12);
    println!("My name is {1:?},{0:?} {1:?}", "Berry", "Straw");
    
    //printing structure 
    println!("Now {:?} will be printed", Structure(3));

    //the only problem is there is no control over how output would look
    println!("Now {:?} will print",Deep(Structure(69)));

    //pretty printing using {:#?}
    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};

    //pretty print
    println!("{:#?}", peter);





}
