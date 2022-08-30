# Personal TODO

- Read and review the rust book.

# Resources

- [The rust book [english]](https://doc.rust-lang.org/book/title-page.html)
- [The rust book [translations]](https://doc.rust-lang.org/book/appendix-06-translation.html)

# Keyboard

## Layout (engram)

#### Guidelines:

- Promote alternating between hands over uncomfortable same-hand transitions.

- Arrange letters so that more frequent bigrams are easier to type.

- Promote little-to-index-finger roll-ins over index-to-little-finger roll-outs.

- Balance finger loads according to their relative strength.

- Avoid stretching shorter fingers up and longer fingers down.

- Avoid using the same finger sequentially.

- Avoid skipping over the home row.

- Assign punctuation to keys in the middle of the keyboard

### Primary keys

```
    Left:            Right:
 -  2  3  -        - 14 15  -  
 5  6  7  8       17 18 19 20  
 9  -  - 12       21  -  - 24
```

The keys `7, 8, 17, 18` have **priority** over the rest of the main keys.

### Secondary keys

```
    Left:            Right:
 1  -  -  4       13  -  - 16
 -  -  -  -        -  -  -  -
 - 10 11  -        - 22 23  -
```

### Assign the most common letters to the most comfortable keys (Norvig's analysis)

Letter frequencies: E, T, A, O, I, N, S, R, H, L, D, C, U, M, F, P, G, W, Y, B, V, K, X, J, Q, Z

### Vowels on the left half

Vowel frequencies: **E**, T, **A, O, I**, N, S, R, H, L, D, C, U, M, F, P, G, W, Y, B, V, K, X, J, Q, Z

Vowel digram frequencies:

```
 OU  24,531,132,241
 IO  23,542,263,265
 EA  19,403,941,063
 IE  10,845,731,320
 AI   8,922,759,715
 IA   8,072,199,471   
 EI   5,169,898,489
 UE   4,158,448,570       
 UA   3,844,138,094   
 AU   3,356,322,923
 UI   2,852,182,384
 OI   2,474,275,212
 EO   2,044,268,477
 OA   1,620,913,259
 OE   1,089,254,517
```

- The digram **EA** places the letter **E** in the middle key of the middle finger and the letter **A** in the middle key of the index finger.

- The digram **IO** places the the letter **I** before the letter **O**.

You are left with the following arragement combinations:

```
- - O -    - - O -    - - - -    
- I E A    I - E A    I O E A
- - - -    - - - -    - - - -
```

### Common consonants on the right half

Consonant frequencies: E, **T**, A, O, I, **N, S, R, H**, L, D, C, U, M, F, P, G, W, Y, B, V, K, X, J, Q, Z

Consonants digram frequencies:

```
 TH 100,272,945,963  3.56% 
 ND  38,129,777,631  1.35%
 ST  29,704,461,829  1.05%
 NT  29,359,771,944  1.04%
 CH  16,854,985,236  0.60%
 NS  14,350,320,288   
 CT  12,997,849,406
 TR  12,006,693,396       
 RS  11,180,732,354   
 NC  11,722,631,112
 RT  10,198,055,461   
```

The digram constraints leave you with the following arragement combinations (satisfying the greatest sum of frequencies):

```
- - - -    - - - -    - - - -    - - - -    - - - -
R T S N    H T S N    H T S R    H T N R    T S N R
- - - -    - - - -    - - - -    - - - -    - - - -
```

#### The 5th most frequent consonant

We will assign the fifth consonant to a vacant key on the left home row 
if there is a vacancy, otherwise to the key below the right index finger
 (any other assignment requires the same finger to type a high-frequency
 bigram). 

The combinations are represented below with the three rows on the left and right side of the keyboard oriented linearly:

```
--O- HIEA ----    ---- RTSN ----
--O- RIEA ----    ---- HTSN ----
--O- NIEA ----    ---- HTSR ----
--O- SIEA ----    ---- HTNR ----
--O- IHEA ----    ---- RTSN ----
--O- IREA ----    ---- HTSN ----
--O- INEA ----    ---- HTSR ----
--O- ISEA ----    ---- HTNR ----
--O- -IEA ----    ---- RTSN H---
--O- -IEA ----    ---- HTSN R---
--O- -IEA ----    ---- HTSR N---
--O- I-EA ----    ---- RTSN H---
--O- I-EA ----    ---- HTSN R---
--O- I-EA ----    ---- HTSR N---
---- IOEA ----    ---- RTSN H---
---- IOEA ----    ---- HTSN R---
---- IOEA ----    ---- HTSR N---
--O- HIEA ----    ---- TSNR ----
--O- IHEA ----    ---- TSNR ----
```

### Assignment of next 7 frequent letters

E, T, A, O, I, N, S, R, H, **L, D, C, U, M, F, P**, G, W, Y, B, V, K, X, J, Q, Z

These letters will go on the remaining slots of the **main keys**. There are $7!$ combinations for each of the above combinations.

### Assignment of the next 8 frequent letters

E, T, A, O, I, N, S, R, H, L, D, C, U, M, F, P, **G, W, Y, B, V, K, X, J**, Q, Z

These leters will go on the 

### Optimization

To arrange the letters, we construct a frequency matrix where we multiply a matrix containing the frerquency of each bigram by our flow and strength matrices to computer a score. The optimization algorithm ranks these letter-key mappings according to a score reflecting ease of typing key pairs and frequency of letter pairs (bigrams). The score is the average of the scores for all possible bigrams in arrangement.

#### Score penalization fractors

Direction:

```
- outward = 0.9: outward roll of fingers from the index to little 
    finger (same hand)
```

Dexterity:

```
- side_above_3away = 0.9
    - index and little finger type two keys, one or more rows apart (same hand)
- side_above_2away = 0.9^2 = 0.81
    - index finger types key a row or two above ring finger key, or
    - little finger types key a row or two above middle finger key (same hand)
- side_above_1away = 0.9^3 = 0.729
    - index finger types key a row or two above middle finger key, or
    - little finger types key a row or two above ring finger key (same hand)
- middle_above_ring = 0.9
    - middle finger types key a row or two above ring finger key (same hand)
- ring_above_middle = 0.9^3 = 0.729
    - ring finger types key a row or two above middle finger key (same hand)
- lateral = 0.9
    - lateral movement of (index or little) finger outside of 8 vertical columns
```

Distance:

```
- skip_row_3away = 0.9       
    - index and little fingers type two keys that skip over home row (same hand)
    - (e.g., one on bottom row, the other on top row)
- skip_row_2away = 0.9^3 = 0.729
    - little and middle or index and ring fingers type two keys that skip over home row (same hand)
- skip_row_1away = 0.9^5 = 0.59049
    - little and ring or middle and index fingers type two keys that skip over home row (same hand)
```

Repretition:

```
- skip_row_0away = 0.9^4 = 0.6561
    - same finger types two keys that skip over home row
- same_finger = 0.9^5 = 0.59049
    - use same finger again for a different key
    - cannot accompany outward, side_above, or adjacent_shorter_above 
```

Strength: Accounted for by the strength matrix (minimun value for the little finger = 0.9)

#### Additional Optimization

If we relax the above initializations and permit further exchange of letters, then we can search for higher-scoring layouts.

```
1. Top rows
2. Bottom rows
3. Top and bottom rows on the right side
4. Top and bottom rows on the left side
5. Top right and bottom left rows
6. Top left and bottom right rows
7. Center of the top and bottom rows on both sides
8. The eight corners
9. Left half of the top and bottom rows on both sides
10. Right half of the top and bottom rows on both sides
11. Left half of non-home rows on the left and right half of the same rows on the right
12. Right half of non-home rows on the left and left half of the same rows on the right
13. Top center and lower sides
14. Top sides and lower center
15. Repeat 1-14
```

### Final layout

```
B Y O U  L D W V Z
C I E A  H T S N Q
G X J K  R M F P    
```

#### Evaluation:

```
1. Evaluate optimized layouts using interkey speed estimates   
2. Evaluate variants of the candidate winner using interkey speed estimates
3. Test sensitivity of the candidate winner to the scoring parameters
```

##### Test 1

we rescored all of the 20 top-scoring layouts optimized from the 20 initialized layouts, and replaced the factor matrix with the inter-key speed matrix. The same two layouts that tied for first place do so again.

##### Test 2

we rescored all of the 5,040 variants of the candidate winner that were tied for first place, replacing the factor matrix with the interkey speed matrix. The candidate winner scored highest.

##### Test 3

we ran a test on the variants of the candidate winner layout to see how robust they are to removal of scoring parameters. We removed each of the 11 scoring parameters one by one and ranked the new scores for the variants. The candidate winner scored highest for 8 of the 11 cases, and second highest for two other cases, demonstrating that this layout is not sensitive to individual parameters.

### Punctuation

Frequency:

```
         Sun:     Malik:   Ruhlen:    Cook:            Xah:
          /1M   N-gram %   /10,000   /1,000       All%  JS%   Py%

.    42840.02      1.151       535     65.3       6.6   9.4  10.3
,    44189.96                  556     61.6       5.8   8.9   7.5
"                  2.284        44     26.7       3.9   1.6   6.2
'     2980.35      0.200        40     24.3       4.4   4.0   8.6
-     9529.78      0.217        21     15.3       4.1   1.9   3.0
()    4500.81      0.140         7                7.4   9.8   8.1
;     1355.22      0.096        22      3.2       3.8   8.6
z                  0.09                   -         -
:     3221.82      0.087        11      3.4       3.5   2.8   4.7
?     4154.78      0.032        14      5.6       0.3
/                  0.019                          4.0   4.9   1.1
!     2057.22      0.013         3      3.3       0.4
_                  0.001                         11.0   2.9  10.5
```

### Add punctuation keys and number keys

We will assign the most frequent punctuation according to Sun, et al 
(2018) to the six keys in the middle two columns: . , “ ‘ - ? ; : () ! _

```
        B Y O U   '  "   L D W V Z
        C I E A   ,  .   H T S N Q
        G X J K   -  ?   R M F P
```

We will use the Shift key to group similar punctuation marks 
(separating and joining marks in the left middle column and closing 
marks in the right middle column):

```
        B Y O U  '(  ")  L D W V Z #$ @`
        C I E A  ,;  .:  H T S N Q
        G X J K  -_  ?!  R M F P
```

**Separating marks (left)**: The comma separates text in
 lists; the semicolon can be used in place of the comma to separate 
items in a list (especially if these items contain commas); open 
parenthesis sets off an explanatory word, phrase, or sentence.

**Joining marks (left)**: The apostrophe joins words as 
contractions; the hyphen joins words as compounds; the underscore joins 
words in cases where whitespace characters are not permitted (such as in
 variables or file names).

**Closing marks (right)**: A sentence usually ends with a
 period, question mark, or exclamation mark. The colon ends one 
statement but precedes the following: an explanation, quotation, list, 
etc. Double quotes and close parenthesis closes a word, clause, or 
sentence separated by an open parenthesis.

**Number keys**: 
The numbers are flanked to the left and right by [square brackets], and 
{curly brackets} accessed by the Shift key. Each of the numbers is 
paired with a mathematical or logic symbol accessed by the Shift key:
