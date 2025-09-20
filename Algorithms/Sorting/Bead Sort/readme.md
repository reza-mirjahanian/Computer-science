Bead Sort, also known as Gravity Sort, is a natural sorting algorithm designed specifically for sorting lists of positive integers by simulating the physical process of beads falling under gravity on parallel rods, similar to an abacus [1][8].

### How Bead Sort Works
- Each integer in the input list is represented by a row of beads suspended on vertical rods, where the number of beads corresponds to the integer's value.
- When gravity is applied, beads "fall" down the rods to the lowest possible position, effectively sorting the numbers as larger values cause more beads to accumulate at the bottom rows.
- After the beads settle, counting the beads in each row from bottom to top yields the sorted list in ascending order [1][8].

### Algorithm Implementation
- The algorithm can be implemented using a 2D matrix where rows represent the integers and columns represent bead positions.
- Beads are marked in the matrix, then iteratively dropped down to the lowest empty positions.
- The sorted output is obtained by counting beads per row after all beads have fallen [1][8].
- Example pseudocode involves initializing the matrix, simulating bead drops, and reading the sorted result [8].

### Complexity
- Time complexity varies by implementation:
  - In idealized physical or hardware models, it can achieve O(n) time by moving beads simultaneously or one row at a time [1][8].
  - In software implementations, especially single-threaded, it tends to be slower, with worst-case time complexity O(S), where S is the sum of the input integers, due to moving beads individually [1][8].
- Space complexity is O(n × m), where n is the number of integers and m is the maximum integer value, due to the matrix representation [1][8].

### Limitations
- Only works with positive integers.
- Not efficient for large or arbitrary data sets in software due to high space and time requirements.
- Best suited for hardware or parallel implementations where beads can be moved simultaneously [1][8].

### Summary
Bead Sort is a conceptually simple, nature-inspired sorting algorithm that leverages the analogy of beads falling under gravity to sort positive integers. While elegant and capable of linear time in hardware, it is generally impractical for software sorting of large or diverse data sets due to its space and time complexity [1][8].

References:
[1] https://en.wikipedia.org/wiki/Bead_sort
[2] https://play.google.com/store/apps/details?id=com.FogwattGames.BeadSort&hl=en_US
[3] https://www.geeksforgeeks.org/dsa/bead-sort-natural-sorting-algorithm/
[4] https://apps.apple.com/il/app/bead-sort/id1500802433
[5] https://apps.apple.com/us/app/bead-sort/id1500802433
[6] https://play.google.com/store/apps/details?id=com.beadsortbs.abs&hl=en_US
[7] https://mathworld.wolfram.com/Bead-Sort.html
[8] https://www.baeldung.com/cs/gravity-sort
[9] https://www.cs.auckland.ac.nz/~mjd/misc/BeadSort5.pdf
