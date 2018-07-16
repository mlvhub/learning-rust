fn main() {

    fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_copy(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'b', 'c', 'd', 'e'];
    let result = largest_copy(&char_list);
    println!("The largest char is {}", result);

    fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for ref item in list.iter() {
            if item > &largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_clone(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'b', 'c', 'd', 'e'];
    let result = largest_clone(&char_list);
    println!("The largest char is {}", result);

    let string_list = vec![String::from("abc"), String::from("def")];
    let result = largest_clone(&string_list);
    println!("The largest string is {}", result);

    fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for ref item in list.iter() {
            if item > &largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_ref(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'b', 'c', 'd', 'e'];
    let result = largest_ref(&char_list);
    println!("The largest char is {}", result);

    let string_list = vec![String::from("abc"), String::from("def")];
    let result = largest_ref(&string_list);
    println!("The largest string is {}", result);

    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: Display
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }


    let result = longest_with_an_announcement("hello", "world!", "The world is huge!");
    println!("The result is {}", result);
}
