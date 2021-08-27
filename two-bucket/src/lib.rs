use std::collections::HashSet;
use std::cmp::min;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct BucketState {
    pub bucket: Bucket,
    pub capacity: u8,
    pub volume: u8
}

impl BucketState {
    fn available(&self) -> u8 {
        self.capacity - self.volume
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct State {
    pub one: BucketState,
    pub two: BucketState,
    pub goal: u8
}

impl State {
    fn complete(&self) -> bool {
        self.one.volume == self.goal || self.two.volume == self.goal
    }

    fn mut_buckets(&mut self, bucket:Bucket) -> (&mut BucketState, &mut BucketState) {
        match bucket {
            Bucket::One => (&mut self.one, &mut self.two),
            Bucket::Two => (&mut self.two, &mut self.one)
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum ActionType {
    Fill,
    Empty,
    Pour
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Action(ActionType, Bucket, State);

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let one = init(Bucket::One, capacity_1, start_bucket);
    let two = init(Bucket::Two, capacity_2, start_bucket);
    let state = State {
        one, two, goal
    };
    match do_solve(state) {
        Some((State, moves)) => {
            None
        }
        _ => None
    }
}

fn init(bucket:Bucket, capacity:u8, starting: &Bucket) -> BucketState {
    let volume = match starting {
        x if x == &bucket => capacity,
        _ => 0
    };
    BucketState {
        bucket,
        capacity,
        volume
    }
}

fn do_solve(init:State) -> Option<(State, u8)> {

    let mut tried:HashSet<Action> = HashSet::new();
    let mut actions:Vec<Action> = next_actions(init);

    while !actions.is_empty() {

        let action = actions.remove(0);

        // if we've hit this point before, skip
        if tried.contains(&action) {
            continue;
        }
        tried.insert(action);

        let Action(action, bucket, state) = action;
        let state = match action {
            ActionType::Pour => pour(bucket, state),
            ActionType::Fill => fill(bucket, state),
            ActionType::Empty => empty(bucket, state),
        };

        if state.complete() {
            return Some((state, 1))
        }

        actions.append(&mut next_actions(state));

    }

    None
}

fn next_actions(state:State) -> Vec<Action> {
    let mut actions = Vec::new();
    for action in [ActionType::Fill, ActionType::Empty, ActionType::Pour] {
        for bucket in [Bucket::One, Bucket::Two] {
            actions.push(Action(action, bucket, state))
        }
    }
    actions
}

fn pour(bucket: Bucket, mut state: State) -> State {
    let (primary, secondary) = state.mut_buckets(bucket);
    let mv = min(secondary.available(), primary.volume);
    primary.volume -= mv;
    secondary.volume += mv;
    state
}

fn empty(bucket: Bucket, mut state: State) -> State {
    let (primary, _) = state.mut_buckets(bucket);
    primary.volume = 0;
    state
}

fn fill(bucket: Bucket, mut state: State) -> State {
    let (primary, _) = state.mut_buckets(bucket);
    primary.volume = primary.capacity;
    state
}


