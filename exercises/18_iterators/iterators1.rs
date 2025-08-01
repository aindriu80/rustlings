// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: Create an iterator over the array.
        let mut fav_fruits_iterator = my_fav_fruits.into_iter();

        assert_eq!(fav_fruits_iterator.next(), Some(&"banana").map(|v| &**v));
        assert_eq!(
            fav_fruits_iterator.next(),
            Some(&"custard apple").map(|v| &**v)
        );
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado").map(|v| &**v));
        assert_eq!(fav_fruits_iterator.next(), Some(&"peach").map(|v| &**v));
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry").map(|v| &**v));
        assert_eq!(fav_fruits_iterator.next(), None);
    }
}
