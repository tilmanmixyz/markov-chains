const TEST_STR: &str = include_str!("../test.txt");

fn main() {
    let markov = markov_chains::Letters::new_from_str(TEST_STR).unwrap();
    let (c, v) = markov.finalize();
    let pairs = markov.analyze_pairs();
    println!("Total: {}, Consonants: {}, Vowels: {}", c + v, c, v);
    println!("Pairs\n\tCC: {}\n\tCV: {}\n\tVC: {}\n\tVV: {}", pairs.cc, pairs.cv, pairs.vc, pairs.vv);
}
