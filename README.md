A substring matching algorithm I came up with on a dime.  

## Basic Idea

1. Divide words by whitespace  
```
"the quick brown fox jumps over the lazy dog" => ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
```

2. Sort words into arrays based on their character count  
```
a3 = ["the", "fox", "dog"]
a4 = ["over", "lazy"]
a5 = ["quick", "brown", "jumps']
 ...
a45 = [] // Pneumonoultramicroscopicsilicovolcanoconiosis is the longest supported word by the algorithm
```

3. Sort each array alphabetically  
```
a3 = ["dog", "fox", "the"]
a4 = ["lazy", "over"]
a5 = ["brown", "jumps", "quick"]
 ...
a45 = []

```

4. Binary search through one specific array to find the target word  

## Reasoning  

- Better than naive substring matching as it eliminates the need to compare words that are different length as they
fundamentally cannot match.  

- Would save considerable amount of time when searching for a specific non frequent word through a large string sample, especially helpful
if that word has a non common character length  

## Limitations  

- Indexing a given document would be fairly computationally expensive, search cannot be performed without indexing  

- Would work best for text that follows the commona lexical structure of the english language, as words as split on a single whitespace  
    `main() => 6 characters`

- Works best for a static, infrequently changing document
