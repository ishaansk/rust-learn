In rust any datatype is immutable by default, i.e. can not be changed
add mut like 
let mut variablename
to make it mutable
rust clears any memory allocated to any memory variable, if you try to call the slice variable outside the function it was defined in, boom, error. Not found in scope.
you can have either only one mutable reference or any number of immutable references to a variable. you can not have both immutable and mutable references to a variable
