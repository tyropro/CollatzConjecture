# Collatz Conjecture

The Collatz Conjecture is a problem in mathematics that has gone a long time without being solved.

The rule goes:
- if the current number is odd
  - multiply the current number by 3
  - then add 1
- if the current number is even
  - divide the current number by 2 

The result number then becomes the next number in the sequence and it carrys on from there. 
(This is my understanding of the Collatz Conjecture. If I have misunderstood something, please let me know.)

This is a Rust program that calculates the sequence of the Collatz Conjecture with a number inputted by the user.
It calculates the sequence all the way to 1 (where the rule gets stuck in a 1, 4, 2, 1 loop) and adds each iteration to a `.csv` file with an ID assigned to each iteration.
You can use a CSV Graph site like [CSV Plot](https://www.csvplot.com) to visualise the results in either a scatter graph or a line graph.