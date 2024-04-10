# IntroToSubstrate_A4

## Description and Running
This is the 4th assignment of Introduction to Substrate, where we completed three coding tasks. To test the projects, first clone the repository:
```
$ git clone https://github.com/YADengUU/IntroToSubstrate_A4
```
and change the directory to the corresponding tasks as explained below.
### Task 1 (Question 8)
We made the traffic lights to have different duration: red has 25 seconds, yellow has 5 seconds, and green has 60 seconds. To see the outcome, when your are in the directory (`cd traffic_light_enum`), just do `cargo run` and it should print out
```
   Compiling traffic_light_enum v0.1.0 (/your_destination_directory/IntroToSubstrate_A4/traffic_light_enum)
    Finished dev [unoptimized + debuginfo] target(s) in 1.31s
     Running `target/debug/traffic_light_enum`
Duration of red light: 25 seconds
Duration of yellow light: 5 seconds
Duration of green light: 60 seconds
```
The time cost to compile might be different depending on the local machine setups.

### Task 2 (Question 9)
This part calculates the sum of a collection of integers (u32) with parameter type &[u32] and returns type Option<u32> or None in the case of overflow. We used the collection with 1) numbers 1 to 10 for a simple test and 2) infinity, e.g., `u32:: MAX`, for testing the overflowing scenario. Similarly, to see the outcome, in the directory `IntroToSubstrate_A4`, do `cd sums_up_collection` and `cargo run`, it should show the following
```
   Compiling sums_up_collection v0.1.0 (/your_destination_directory/IntroToSubstrate_A4/sums_up_collection)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/sums_up_collection`
Sum of numbers1: 55
Sorry, overflow!
```

### Task 3 (Question 10)
This part calculates the area of geometric shapes, of which the trait bound for their corresponding parameters such as the side width, height, or radius, is restricted to be `f64`. We have the following shapes: circle (tested with `radius=1.5`), triangle (`base=3.0` and `height=4.0`), reactangle (`width1=1.5` and `width2=2.5`, as well as square (`side=5.5`). In the same way - `cd compute_areas`, `cargo run`, you should be able to see
```
   Compiling compute_areas v0.1.0 (/your_destination_directory/IntroToSubstrate_A4/compute_areas)
    Finished dev [unoptimized + debuginfo] target(s) in 1.15s
     Running `target/debug/compute_areas`
Area: 7.0685834705770345
Area: 6
Area: 3.75
Area: 30.25
```
