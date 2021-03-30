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


## Working with Strings
### Question
> Split a string into a vector of its constituent words.
### Solution
```
let a = String::from("hello world foo bar);
let b: Vec<&str> = a.split(" ").collect();
let c: Vec<&str> = a.split_whitespace().collect();
let d: Vec<String> = a.split_whitespace().map(|c|{c.to_string()}).collect();


```
### Question
> Split a string into a vector of its characters.

### Solution
```
let a = String::from("hello");
let b: Vec<String> = a.chars.map(|c|{String::from(c)}).collect();
println!("{:?}", b);
```

### Question
> Convert a vector of strings into a string?

### Solution
```
b.into_iter().collect::<String>()
```
Note: `into_iter()` will consume `b`, `iter()` will give you an iterator over references, which will not collect into a String so easily, since it is a bundle of references.


### Question
> Convert a String into a number
### Solution
```
let a = String::from("123");
let num: i32 = c.parse().unwrap();
```
### Question
> Pattern match on a char type
### Solution
```
match c.to_string().as_str() {
    "a" => {},
    "b" => {},
    "c" => {},
    _ => {}
}
```
## Accessing `Rc<RefCell<T>>`

Some of Leetcode's binary tree questions are not as idiomatic as they could be. For a traversal of a Binary tree question I found myself wrestling with Leetcode's implementation, their tree nodes have this type: `Option<Rc<RefCell<T>>>`, when `Option<Box<T>>` would likely suffice.

[Unlike `Box<T>` which doesn't require anything special to access its contents](https://doc.rust-lang.org/book/ch15-01-box.html) - it seems hard to unwrap and access T from an `Option<Rc<RefCell<T>>>`.

Googling for an answer I found that I'm not the first to encounter this:
> [You really don't want to try to remove the value from the Option, the Rc or the RefCell via unwrap / try_unwrap / into_inner. Instead, pattern match on the Option and then call borrow on the RefCell to get a reference to the T.](https://stackoverflow.com/questions/54012660/unwrap-and-access-t-from-an-optionrcrefcellt)

```
let my_val: Option<Rc<RefCell<T>> = Some(Rc::new(RefCell::new("hello")));

match my_val {
    Some(v) => {
        let v = v.borrow();
        // we now have a reference to "hello" to work with!
    },
    None() => {}
}
```
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

## Intersection of two arrays 2
### Question
> Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.
### Solution
#### First attempt
```
use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn vec_to_hashmap(arr: &Vec<i32>) -> HashMap<i32, Vec<i32>> {
        
        let mut hm: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut count = 0;
        
        for n in arr {
            match hm.entry(*n) {
                Entry::Occupied(mut l) => {
                    l.get_mut().push(count);
                },
                Entry::Vacant(_) => {
                    hm.insert(*n, vec![count]);
                }
            }
            count +=1;
        }
        hm
        
    }
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        
        let mut hm_1: HashMap<i32, Vec<i32>> = Solution::vec_to_hashmap(&nums1);
        let mut hm_2: HashMap<i32, Vec<i32>> = Solution::vec_to_hashmap(&nums2);
        
        let mut result: Vec<i32> = vec![];
        
        for num in nums1 {
            if hm_1.get(&num).is_some() {
                let mut p = hm_1.remove(&num).unwrap();
                match hm_2.remove(&num) {
                    Some(mut q) => {
                        if q.len() > p.len() {
                            for i in p {
                                result.append(&mut vec![num]);
                            }
                        } else {
                            for i in q {
                                result.append(&mut vec![num])
                            }
                        }
                    },
                    None => {}
                }
            }
        }

        result
    }
}
```
### Notes
When coming up with this solution I found the following useful methods:
- `HashMap::remove(k)`, which removes a key from an hash map and returns its value.

When coming up with this solution I encountered these pain points.
- I wanted to find the shorter of the two lists.
    - Use the method [`min(x,y)`](https://doc.rust-lang.org/std/cmp/fn.min.html) which compares and returns the minimum of two values.
- I wanted to convert a `Vec` to a `HashMap` in a less long winded way.
    - I found the method [`or_insert`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert) on HashMap
    ```
    let mut letters = HashMap::new();

    for ch in "a short treatise on fungi".chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }
    ```

