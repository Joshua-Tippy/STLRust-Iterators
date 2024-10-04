
fn main() {
/*
    
    So we have gone over some rust basics like lifetimes, ownership, ect... and now we will be taking a closer look at a functional feature of rust that is Iterators.
    But before we dive into some examples we need to define some terms.  



DEFINITIONS --------------------------------

    Iterator: A design pattern that allows sequential access to elements in a collection without exposing the underlying structure.

    Collection: A data structure that holds multiple values, such as arrays, vectors, hash maps, or sets.

    Lazy Evaluation: A strategy where computation is deferred until the result is actually needed, which can improve performance.

    Method Chaining: A technique that allows calling multiple methods on an object in a single statement, often used with iterators.

    Closures: Anonymous functions that can capture their environment, commonly used as arguments for iterator methods. Example: thing.map(|a|a * 4).collect()  Note that this is also an example of Lazy evaluation.  the (a * 4) in the map will not be performed unless next() is called

    Consume: The process of using an iterator to produce values, which results in the iterator being exhausted.

    Mutable vs. Immutable Iterators: Understanding the difference between iterators that can modify the underlying collection and those that cannot.

    Filters: A way to create a new iterator that only includes elements that meet specific criteria.

    Mapping: Transforming each element of an iterator into a new form, creating a new collection with the transformed values.

    Next() Method: The method used to retrieve the next item from an iterator, fundamental to understanding how iterators operate. 
    
---------------------------------------------

    Now lets look at out first iterator.  
*/ 
    let numbers = vec![1, 2, 3, 4];
    let doubled = numbers.iter().map(|x| {x * 2}).collect::<Vec<i32>>();  //::<Vec<i32>>
    println!("Ex 1: {:?}", doubled); // Output: [2, 4, 6, 8]
/*
    Here, the .iter() method is creating a new iterator from the Vec<i32>.  Note that when using the .iter() method the original vector is not consumed and can still be used.
    the iterator that was created contains a reference to the data within the vec, so the numbers vec is still valid event after calling map().collect().  

    Next the .map() is what is called a closure, it is denoted by the double pipe syntax.  this will do nothing untill .collect() is called.  once .collect() is called
    .next() is called until the end is reached and the operations defined in .map() are performed on each value within the iterator.  

    note that type annotations are needed for the output of the iterator.  the type cannot be inferred. 
*/







    //this line only prints empty lines to the console.  it uses the repeat iterator which will repeat infinitely, in combination with the .take method which will limit the number of iterations.
     std::iter::repeat(()).take(25).for_each(|_| println!());
     println!("examples that highlight the differences between .iter(), .into_iter(), and .iter_mut()------------------------");
//Here are some examples that highlight the differences between .iter(), .into_iter(), and .iter_mut().
//-------------------------------------------------------
    {
        println!("\n\nExample 1: ");
        let vec = vec![1, 2, 3];
        let iter = vec.iter(); // `vec` is not consumed here
        
        let doubled: Vec<i32> = iter.map(|x| x * 2).collect();
        
        // `vec` can still be used here but cannot be changed while iter exists.  this is because iter holds a reference to `vec`.  
        println!("Original Vec: {:?}", vec); // Outputs: [1, 2, 3]
        drop(vec); //doubled is still valid after dropping vec and is not dependent on the original vec
        println!("Doubled Vec: {:?}", doubled); // Outputs: [2, 4, 6]
    }
//-------------------------------------------------------








//-------------------------------------------------------
    {
        println!("\n\nExample 2: ");
        let vec = vec![1, 2, 3];
        let iter = vec.into_iter(); // `vec` is consumed here
        // `vec` cannot be used here anymore; this would cause a compile-time error
        //println!("Original Vec: {:?}", vec);

        let doubled: Vec<i32> = iter.map(|x| x * 2).collect();
        println!("Doubled Vec: {:?}", doubled);

        //vec still cannot be used here even after consuming the iterator
        //println!("Original Vec: {:?}", vec);
    }
//-------------------------------------------------------










//-------------------------------------------------------
    {
        println!("\nExample 3: ");
        let mut vec = vec![1, 2, 3];
        let iter = vec.iter_mut();

        //we cannot reference the original vec after creating the iterator becuase it holds a mutable reference to vec.  
        //let newvec: &Vec<i32> = &vec;

        iter.for_each(|v| *v *= 2);

        //for v in iter {
        //    *v *= 2;
        //}

        //we can reference the original vec after consuming the iterator.  
        //let newvec: &Vec<i32> = &vec;

        // Use iter_mut to iterate over mutable references of the elements
        // vec still exists and is usable but has now been modified.
        println!("Original Vec: {:?}", vec); // Outputs: [2, 4, 6]
    }
//-------------------------------------------------------










    std::iter::repeat(()).take(25).for_each(|_| println!());
    println!("custom iterator impl Example 1 ----------------------------------------------------------------");
    //how to implement iterator trait on a custom type example 1
    {
        struct MyStruct {
            vec: Vec<i32>, 
        }
        impl Iterator for MyStruct {
            type Item = i32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.vec.len() > 0 {
                    Some(self.vec.remove(0))
                } else {
                    None
                }
            }
        }

        let mut m = MyStruct { vec: vec![1, 2, 3, 4] };
        
        //you can perform other operations on this, but i kept it simple for this example
        m.into_iter().for_each(|v| println!("{:?}", v));
    }











    println!("\n\n\n\ncustom iterator impl Example 2 ----------------------------------------------------------------");
    //how to implement iterator trait on a custom type example 2
    {
        struct Person {
            firstname: String,
            lastname: String,
            occupation: String,
        }
        //we need to create a custom iterator here in order to iterate through the fields in Person
        struct PersonIterator {
            values: Vec<String>
        }
        //here we implement the Iterator trait for our custom iterator so rust recognises it as one
        impl Iterator for PersonIterator {
            type Item = String;
            fn next(&mut self) -> Option<Self::Item> {
                if self.values.len() > 0 {
                    Some(self.values.remove(0))
                } else {
                    None
                }
            }
        }
        //next we need to implement IntoIterator for Person.  
        //this is so that when we call .intoiter() on an instance of Person it knows to use our custom iterator.
        impl IntoIterator for Person {
            type Item = String;
            type IntoIter = PersonIterator;
            fn into_iter(self) -> Self::IntoIter {
                PersonIterator { 
                    values: vec! [
                        self.firstname,
                        self.lastname,
                        self.occupation,
                    ]
                }
            }
        }   
            let person = Person {
                firstname: "Joshua".to_owned(),
                lastname: "Tippy".to_owned(),
                occupation: "Software Engineer".to_owned()
            };


            person.into_iter().for_each(|v| println!("{:?}", v));
    }

    



    



    //--------------------------------------------------------------------------------------------------------------------------------------
    std::iter::repeat(()).take(10).for_each(|_| println!());
    //Examples of more complex iterators
    //fold
    use std::collections::HashMap;
    {
        println!("Iter Method Example 1: ");
        let words = vec![
            "apple", "banana", "apple", "orange", 
            "banana", "apple", "kiwi", "orange",
        ];

        // Use fold to create a frequency map
        let frequency: HashMap<&str, usize> = words.iter().fold(HashMap::new(), |mut acc, &word| {
            *acc.entry(word).or_insert(0) += 1; // Increment the count for the word
            acc // Return the accumulator
        });


        // Print the word frequencies
        for (word, count) in frequency {
            println!("{}: {}", word, count);
        }
    }
    //--------------------------------------------------------------------------------------------------------------------------------------






    //--------------------------------------------------------------------------------------------------------------------------------------
    std::iter::repeat(()).take(5).for_each(|_| println!());
    //zip
    println!("Iter Method Example 2: ");
    //if we had two collections we wanted to iterate over simultaneously we could use .zip()

    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];
    let people: Vec<(&str, u32)> = names.into_iter().zip(ages).collect();
    println!("{:?}", people); // Output: [("Alice", 30), ("Bob", 25), ("Charlie", 35)]

    //note that we are using .into_iter() so names and ages are both no longer usable after creating our iterator
    //println!("{:?}", ages); 
    //--------------------------------------------------------------------------------------------------------------------------------------






    //--------------------------------------------------------------------------------------------------------------------------------------
    std::iter::repeat(()).take(5).for_each(|_| println!());
    //chain
    println!("Iter Method Example 3: ");
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let combined: Vec<i32> = a.into_iter().chain(b).map(|x| x * 2).collect();
    println!("{:?}", combined); // Output: [1, 2, 3, 4, 5, 6]

    //println!("{:?}", b); 

    //--------------------------------------------------------------------------------------------------------------------------------------






    //--------------------------------------------------------------------------------------------------------------------------------------
    std::iter::repeat(()).take(5).for_each(|_| println!());
    //peekable and peek
    //this example is technically broken, it shouldnt drop the last element in the collection.  
    //but its still a good use case for peekable and peek.  
    println!("Iter Method Example 4: ");
    {
        let mut vec = vec![2, 1, 2, 8, 5, 7, 8, 5, 6, 4, 8, 8, 9, 10];        
        println!("before removing values: {:?}", vec);
        vec.sort();
        let mut iter = vec.iter_mut().peekable(); // Create a peekable iterator

        let mut results: Vec<i32> = Vec::new();
        while let Some(value) = iter.next() {
            // Peek at the next value without consuming it
            if let Some(next_value) = iter.peek() {
                // if the next value isnt the same as the current value we keep it. this gets us all the unique values
                if *next_value != value {
                    results.push(*value);
                }
            } else {
                println!("No next value (end of iterator).");
            }
        }
        println!("after removing values: {:?}", results);
    }
    //--------------------------------------------------------------------------------------------------------------------------------------
















    std::iter::repeat(()).take(25).for_each(|_| println!());
    println!("Examples of simple useful combiators ---------------------------------------------------------------");
    //Examples of simple useful combiators:
    //Map
    let numbers = vec![1, 2, 3, 4];
    let aftermap: Vec<i32> = numbers.iter().map(|&x| {
        let mut i = x * 2;
        i *= i;
        return i;
    }).collect();
    println!("Ex 1: {:?}", aftermap); // Output: [2, 4, 6, 8]
    println!("Ex 1: {:?}", numbers); // Output: [2, 4, 6, 8]



    std::iter::repeat(()).take(5).for_each(|_| println!());
    //Rev
    let numbers = vec![1, 2, 3, 4, 5];
    let reversed: Vec<i32> = numbers.into_iter().rev().collect();
    println!("\nEx 2: {:?}", reversed); // Output: [5, 4, 3, 2, 1]




    std::iter::repeat(()).take(5).for_each(|_| println!());
    //Filter    
    let numbers = vec![1, 2, 3, 4, 5];
    let evens: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0 || x == 5).collect();
    println!("\nEx 3: {:?}", evens); // Output: [2, 4, 5]




    std::iter::repeat(()).take(5).for_each(|_| println!());
    //Filter_map
    let strings = vec!["1", "2", "foo", "4"];
    let numbers: Vec<i32> = strings.into_iter().filter_map(|s| s.parse().ok()).collect();
    println!("\nEx 4: {:?}", numbers); // Output: [1, 2, 4]




    std::iter::repeat(()).take(5).for_each(|_| println!());
    //Flat_map
    let words = vec!["hello world", "foo bar"];
    let characters: Vec<char> = words.into_iter().flat_map(|s| s.chars()).collect();
    println!("Ex 5: {:?}", characters); // Output: ['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd', 'f', 'o', 'o', ' ', 'b', 'a', 'r']




    std::iter::repeat(()).take(5).for_each(|_| println!());
    //Enumerate
    print!("Ex 6: ");
    let names = vec!["Alice", "Bob", "Charlie"];

    names.iter().enumerate().for_each(|(i, e)| println!("Index: {}, Name: {}", i, e));

    //for (index, name) in names.iter().enumerate() {
    //    println!("Index: {}, Name: {}", index, name);
    //}
    // Output:
        // Index: 0, Name: Alice
        // Index: 1, Name: Bob
        // Index: 2, Name: Charlie


    std::iter::repeat(()).take(5).for_each(|_| println!());
    //Take
    let numbers = vec![1, 2, 3, 4, 5];
    let first_three: Vec<i32> = numbers.into_iter().take(3).map(|e| e * 2).collect();
    println!("Ex 7: {:?}", first_three); // Output: [1, 2, 3]




    std::iter::repeat(()).take(5).for_each(|_| println!());
    //Skip
    let numbers = vec![1, 2, 3, 4, 5];
    let skipped_first_two: Vec<i32> = numbers.into_iter().skip(2).collect();
    println!("Ex 8: {:?}", skipped_first_two); // Output: [3, 4, 5]




    std::iter::repeat(()).take(5).for_each(|_| println!());
    //Sum
    let numbers = vec![1, 2, 3, 4];
    let total: i32 = numbers.iter().sum();
    println!("Example 9: {}", total); // Output: 10




    std::iter::repeat(()).take(5).for_each(|_| println!());
    //Product 
    let numbers = vec![1, 2, 3, 4];
    let product: i32 = numbers.iter().product();
    println!("Example 10: {}", product); // Output: 24  




    std::iter::repeat(()).take(5).for_each(|_| println!());
    //max and min   
    println!("Example 11: ");
    let numbers = vec![1, 2, 3, 4, 5];
    let max = numbers.iter().max();
    let min = numbers.iter().min();
    println!("Max: {:?}", max); // Output: Max: Some(5)
    println!("Min: {:?}", min); // Output: Min: Some(1)




    std::iter::repeat(()).take(5).for_each(|_| println!());
    //all and any
    println!("Example 12: ");
    let numbers: Vec<i32> = vec![1, 2, 3, 4];
    let all_even = numbers.iter().all(|&x| x % 2 == 0);
    let any_odd = numbers.iter().any(|&x| x % 2 != 0);
    println!("All even: {}", all_even); // Output: All even: true
    println!("Any odd: {}", any_odd); // Output: Any odd: false


    /* 
    let string = String::from("
    Joshua,Tippy,05/10/2003
    John,Doe,09/26/2001
    Jane,Doe,04/20/1985");

    let people: Vec<Person> = string
        .lines() // Get an iterator over the lines
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            // Only create a People struct if we have exactly 3 parts
            if parts.len() == 3 {
                Some(Person {
                    first_name: parts[0].trim().to_string(),
                    last_name: parts[1].trim().to_string(),
                    birthday: parts[2].trim().to_string(),
                })
            } else {
                // Return None to filter out invalid lines
                None
            }
        })
        .collect(); // Collect the results into a Vec<People>


    println!("{:?}", people);
    */
}
