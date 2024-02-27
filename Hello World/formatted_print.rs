//Printing in rust is handled using series of macros.

fn main()
{
    //{} is used a placeholder for arguments for any formatted print statement
    println!("Hello, {}","world");
    
    //Positional arguments can also be used . position index starts at 0
    println!("{0}, this is {1}. {1},this is {0}.","Alice","Bob");

    //Named arguments can also be used 
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb="jumps over");

    //Different formatting can be invoked by specifying the format character after ":"
    println!("Base 10: {}",69420);
    println!("Base 2 (binary): {:b}",69420);
    println!("Base 8 (octal): {:o}",69420);
    println!("Base 16 (hexadecimal): {:x}",69420);
    println!("Base 16 (hexadecimal): {:X}",69420);

    //justification of fromatted text
    println!("{number:>8}", number=69);

    //padding numbers with extra zeros 
    //right adjust
    println!("{number:0>6}", number=1);
    //left adjust
    println!("{number:0<6}", number=1);

    //using named argument in the format specifier 
    println!("{number:0>width$}",number=12, width=69);
    
    println!("My name is {0}, {1} {0}.","Berry","Straw");
    
    //struct does not implement fmt::Display
    #[allow(dead_code)] 
    struct Structure(i32);

    //println!("This struct '{}' won't print...", Structure(12));

    let number: f64 =1.0;
    let width: usize =5;
    println!("{number:>width$}");
    
    let pi: f32 = 3.1415926536;
    println!("pi is roughly {:.3}",pi);





}
