# README #

This is my little toy project. It is currently on it's fifth iteration. I have been using it as 
a  way to teach myself the Rust programming language. In short it is supposed to be an Uber Eats kind of clone

### Five ###
This folder represents my thinking on the fifth iteration of the
project, it is currently not really functional. I will most likely move to a 
sixth iteration as I am getting better ideas from reading through code from open source projects
like Tokio, Rocket, mio and Actix-Web.

### Yabonne-server ###

This is the fourth iteration of the project and the only runnable project last I checked. It exposes one POST rest api 
that allows for the registration of buyers. A sample post reques can be found the tests folder as register-buyer.json. 
There is also a register-buyer.txt which contains a sample apache bench command I used to see how the server performed 
under load specifically it's memory and CPU usage. Coming from a JVM(Scala and Java) background it is always a joy to watch
the system statistics for Rust and Go programs.


