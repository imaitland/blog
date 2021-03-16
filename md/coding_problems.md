+++
title = "Coding Problems"
id = "coding_problems"
author = "iain maitland"
description = "Rust related coding problems and answers"
date = 2021-02-24
tag = "rust"
image = "/assets/og-image-512x512.png"
icon = "⁉️"
draft = false
+++

# Coding Problems

This is where I'll be keeping track of problem questions as I answer them. I've found that they're an effective way to learn a new language, and crack open some new neurons!

Also on [leetcode](https://leetcode.com/gl2748/)

## Group Anagrams
### Question
> Given an array of strings strs, group the anagrams together. You can return the answer in any order. An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

### Solution
```
use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    
    pub fn get_key (s: &String) -> String {
        let mut x: Vec<char> = s.chars().collect();
        x.sort();
        let mut y = x.iter().collect::<String>();
        y
    }
    
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        
        let mut dict: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let key = Solution::get_key(&s);
            match dict.entry(key.to_owned()) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(s)
                },
                Entry::Vacant(e) => {
                    dict.insert(key, vec![s]);
                }
            }
        }
        
        let mut result = vec![];
        
        for (k, v) in dict {
            result.push(v);
        }
        
        result
        
    }
}
```

### Notes

The key here is understanding that all anagrams look the same when you sort their individual characters, for example eat and tea, both become aet. We therefore use the sorted version of a word as the key for our HashMap, thereby guaranteeing that anagrams for a word will all be keyed the same.

Generally it is good to note that finding the correct key for a HashMap can be very helpful.

### Rust takeaways:
 - You can pattern match on a HashMap entry, to check if it is vacant or occupied.
 - To split a `String` you can do: `some_string.chars().collect();`
 - You can `collect` an iterable to a type, for example `some_vector.iter().collect::<String>();` turns a `Vector` of characters into a `String`