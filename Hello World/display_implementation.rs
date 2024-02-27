//Importing the fmt module 
use std::fmt;

struct Structure(i32);

//to use '{}' marker, the trait 'fmt::Display' must be implemented manually.
fn main()
{
  impl fmt::Display for Structure {
       //defining the signature 
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
   }
}
