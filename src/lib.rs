use itertools::Itertools;
use sscanf::sscanf;

#[derive(Debug)]
struct CrateStacks {
    stacks: Vec<Vec<char>>,
}

impl CrateStacks {
    fn new(input: &str) -> Self {
        let rows: Vec<_> = input
            .split('\n')
            .map(|x| {
                x.chars().collect::<Vec<char>>()
                    .chunks(4) // Get groups like " [N]"
                    .map(|x| x[1]) // Pull out the crate, e.g. 'N'
                    .collect::<Vec<_>>()
            }) // extract the crates from each row
            .rev().skip(1).collect(); // Rows from bottom to top, discarding numbers row

        let mut crate_stacks = CrateStacks{stacks: vec![Vec::new(); rows[0].len()]};

        for row in rows{
            for (i, crt) in row.into_iter().enumerate(){
                if crt != ' '{
                    crate_stacks.stacks[i].push(crt);
                }
            }
        }

        crate_stacks
    }

    // expects 0-indices
    fn move_crate(&mut self, from: &usize, to: &usize) {
        let moved_crate = self.stacks[*from].pop().expect("Should not try to move from an empty stack.");
        self.stacks[*to].push(moved_crate);
    }

    fn move_crates(&mut self, from: &usize, to: &usize, n: &usize){
        let mut moved_stack = Vec::new();

        for _i in 0..*n{
            moved_stack.push(self.stacks[*from].pop().expect("Should not try to move from an empty stack"));
        }
        for _i in 0..*n{
            self.stacks[*to].push(moved_stack.pop().unwrap());
        }
    }
}

/// Finds the top crates in each stack and returns a string representing these
/// Uses the CrateMover 6000
/// # Examples
/// ```
/// use std::fs;
/// use advent_of_code_2022_05::find_top_crates_6000;
///
/// let contents = fs::read_to_string("example-input.txt").unwrap();
/// assert_eq!(find_top_crates_6000(&contents), "CMZ");
/// ```
pub fn find_top_crates_6000(input: &String) -> String {
    let (starting_stacks, raw_instructions) = input.split("\n\n").next_tuple().unwrap();
    let mut stacks = CrateStacks::new(starting_stacks);
    let steps: Vec<_> = raw_instructions.split('\n').filter(|x| !x.is_empty()).collect();

    for step in steps{
        let (n, from, to) = sscanf!(step, "move {} from {} to {}", usize, usize, usize).expect("input should be correctly formatted");

        for _i in 0..n{
            stacks.move_crate(&(from-1), &(to-1)); // subtract 1 to 0-index
        }
    } // Apply instructions

    stacks.stacks.iter().map(|x| x.last().expect("Every stack should have at least one crate")).collect::<String>()
}

/// Finds the top crates in each stack and returns a string representing these
/// Uses the CrateMover 6001
/// # Examples
/// ```
/// use std::fs;
/// use advent_of_code_2022_05::find_top_crates_6001;
///
/// let contents = fs::read_to_string("example-input.txt").unwrap();
/// assert_eq!(find_top_crates_6001(&contents), "MCD");
/// ```
pub fn find_top_crates_6001(input: &String) -> String {
    let (starting_stacks, raw_instructions) = input.split("\n\n").next_tuple().unwrap();
    let mut stacks = CrateStacks::new(starting_stacks);
    let steps: Vec<_> = raw_instructions.split('\n').filter(|x| !x.is_empty()).collect();

    for step in steps{
        let (n, from, to) = sscanf!(step, "move {} from {} to {}", usize, usize, usize).expect("input should be correctly formatted");

        stacks.move_crates(&(from-1), &(to-1), &n);
    } // Apply instructions

    stacks.stacks.iter().map(|x| x.last().expect("Every stack should have at least one crate")).collect::<String>()
}
