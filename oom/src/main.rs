fn main() {
    println!("Hello, world!");
}




#[cfg(test)]
mod tests {

    const MEMORY_LIMIT: usize = 8_000_000_000;
    //const MEMORY_LIMIT: usize = 32_000_000;

    #[test]
    fn oom() {
        // allocate 32GB of memory and hope for the best
        println!("Starting to allocate {}", MEMORY_LIMIT);
        let mut t : Vec<usize> = vec![];
        let a = vec![0; 100000];

        println!("Allocated; Starting to write {}", MEMORY_LIMIT);
        for i in 0..MEMORY_LIMIT {
            t.extend(a.iter());
        }
        println!("Done");
    }

}
