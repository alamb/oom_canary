fn main() {
    println!("Hello, world!");
}




#[cfg(test)]
mod tests {

    //const MEMORY_LIMIT: usize = 8_000_000_000;
    const MEMORY_LIMIT: usize = 32_000_000_000;

    #[test]
    fn oom() {
        // allocate 32GB of memory and hope for the best
        println!("Starting to allocate {}", MEMORY_LIMIT);
        let mut t : Vec<_> = vec![];
        let a: Vec<usize> = vec![0; 100000];

        println!("Allocated; Starting to write {}", MEMORY_LIMIT);
        let mut total = 0;
        while total < MEMORY_LIMIT {
            let a = a.clone();
            total += std::mem::size_of::<Vec<usize>>() + (a.capacity() * std::mem::size_of::<usize>());
            t.push(a);
        }
        println!("Done");
    }

}
