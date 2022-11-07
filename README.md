# littlerustcomputer
I wanna make something like the Little Man Computer but in Rust. Seems like a good first project with the language as well as a good way to learn more low level stuff about computers.

Works back one commit but only with `i32`. I am trying to change the system so it can work by utilising `std::any::Any` by changing the memory registers (and by extension, anything else that uses it) to be able to work with `Box<dyn Any>` however I feel like I have or will hit a wall with it and will probably need to fully rewrite rather than just refactor. Who knows.
