#[allow(unused)]
#[allow(non_snake_case)]

use rand::prelude::*;

const POPULATION_SIZE: usize = 100;
const HAZARD_RATE: f64 = 0.6; // Mutation Rate
const BOARD_SIZE: usize = 5;

#[derive(Clone, Debug)]
struct Board {
    queens: Vec<usize>, // Board structure
}

impl Board {
    fn new(size: usize) -> Board {
        let mut queens = Vec::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            queens.push(rng.gen_range(0..size));
        }
        Board { queens }
    }

    fn fitness(&self) -> usize {
        let mut conflicts = 0;
        for (i, &qi) in self.queens.iter().enumerate() {
            for (j, &qj) in self.queens.iter().enumerate() {
                if i != j {
                    if qi == qj || (qi as isize - qj as isize).abs() == (i as isize - j as isize).abs() {
                        conflicts += 1;
                    }
                }
            }
        }
        conflicts
    }

    fn crossover(&self, other: &Board) -> Board {
        let mut rng = rand::thread_rng();
        let crossover_point = rng.gen_range(0..self.queens.len());
        let mut new_queens = Vec::with_capacity(self.queens.len());
        new_queens.extend_from_slice(&self.queens[..crossover_point]);
        new_queens.extend_from_slice(&other.queens[crossover_point..]);
        Board { queens: new_queens }
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.queens.len() {
            if rng.gen::<f64>() < HAZARD_RATE {
                self.queens[i] = rng.gen_range(0..self.queens.len());
            }
        }
    }    

    fn print_board(&self) {
        let size = self.queens.len();
        for row in 0..size {
            for col in 0..size {
                if self.queens[row] == col {
                    print!(" Q ");
                } else {
                    print!(" - ");
                }
            }
            println!();
        }
        println!();
    }
}

fn roulette_wheel_selection(population: &[Board]) -> Board {
    let total_fitness: usize = population.iter().map(|board| board.fitness()).sum();
    let mut rng = rand::thread_rng();
    let mut slice = rng.gen_range(0..=total_fitness);
    let mut index = 0;
    while slice > population[index].fitness() {
        slice -= population[index].fitness();
        index += 1;
    }
    population[index].clone()
}

fn main() {
    let mut population: Vec<_> = (0..POPULATION_SIZE)
        .map(|_| Board::new(BOARD_SIZE)) 
        .collect();

    let mut generation = 1;

    loop {
        // Sort population by fitness
        population.sort_by_key(|board| board.fitness());

        // Print best solution
        println!("Generation {}: Fitness: {}", generation, population[0].fitness());
        population[0].print_board();

        // Check for solution
        if population[0].fitness() == 0 {
            println!("Solution found in generation {}", generation);
            break;
        }

        // Create new generation
        let mut new_population = Vec::with_capacity(POPULATION_SIZE);
        while new_population.len() < POPULATION_SIZE {
            let parent1 = roulette_wheel_selection(&population);
            let parent2 = roulette_wheel_selection(&population);
            let mut child = parent1.crossover(&parent2);
            child.mutate();
            new_population.push(child);
        }
        population = new_population;
        generation += 1;
    }
}

