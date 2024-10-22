fn countingValleys(steps: i32, path: &str) -> i32 {
    struct State {
        level: i32,
        count: i32,
    }

    fn is_crossed(prev_level: i32, curr_level: i32) -> bool {
        prev_level < 0 && curr_level == 0
    }

    impl State {
        fn new(level: i32, count: i32) -> State {
            State { level, count }
        }
        fn fresh() -> State {
            State::new(0, 0)
        }
        fn next(&self, cur_level: i32, crossed: bool) -> State {
            let delta = if crossed { 1 } else { 0 };
            State::new(cur_level, self.count + delta)
        }
        fn up(&self) -> State {
            let cur_level = self.level + 1;
            self.next(cur_level, is_crossed(self.level, cur_level))
        }
        fn down(&self) -> State {
            let cur_level = self.level - 1;
            self.next(cur_level, is_crossed(self.level, cur_level))
        }
    }

    fn logic(st: State, c: char) -> State {
        match c {
            'U' => st.up(),
            'D' => st.down(),
            _ => st,
        }
    }

    let s = path
        .chars()
        .fold(State::fresh(), logic);

    s.count
}