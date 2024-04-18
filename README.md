6 Degrees of Separation in Marvel Universe | Rust Dec 2023

Developed a Rust-based application to analyze the Marvel Universe dataset, implementing a Breadth-First Search (BFS) algorithm to
determine connections between characters based on their appearances in comics

Utilized BFS to identify the most prolific Marvel character in terms of comic appearances and character interactions via parsing and
quantifying an online CSV dataset, showcasing proficiency in data processing and algorithmic analysis in the Rust programming language

For this project I utilized a Marvel Universe dataset containing every marvel hero and the comics they have appeared in. 
My intent was to determine which characters are the most prominent in all of the comics and use the 6 degrees of separation algorithm to see which characters 
can be reached with 6 degrees, the average amount of connections between each character, and more. 
I really love the Marvel Universe and was interested in determining which heroes are the most important and most prominent. 

As far as the output is concerned, I was able to gain a lot of knowledge from the dataset. 
The first output we see is the top five heroes who have been in the highest percentages of comics. 
This is interesting considering the hero with the most connections to other characters is Spider-Man, yet he is not on the list of the top 5. 
I believe this is the case because comics containing Spider-Man usually include many hero families due to his connection with Tony Stark and the avengers. 
Also, in many comics Spider-Man appears as a supporting character to the primary hero due to his popularity among fans. 
Also, I took the time to calculate the percentage of heroes connected within 6 degrees which came out to be 32.77% of characters; this was done using the 6 degrees of separation algorithm on all characters and their neighbors. 
Finally, the last calculation is used to determine the average number of connections each superhero has, which evened out to 26.65, so almost 27 connections per character.

