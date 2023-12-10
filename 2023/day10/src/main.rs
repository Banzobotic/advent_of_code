fn a() {
    fn do_move(pos: (i64, i64), dir: (i64, i64), current: char) -> (i64, i64) {
        match dir {
            (-1, 0) => match current {
                '|' => (pos.0 - 1, pos.1),
                '7' => (pos.0, pos.1 - 1),
                'F' => (pos.0, pos.1 + 1),
                _ => unreachable!(),
            },
            (0, 1) => match current {
                '|' => (pos.0 - 1, pos.1),
                '7' => (pos.0, pos.1 - 1),
                'F' => (pos.0, pos.1 + 1),
                _ => unreachable!(),
            },
            (1, 0) => match current {
                '|' => (pos.0 - 1, pos.1),
                '7' => (pos.0, pos.1 - 1),
                'F' => (pos.0, pos.1 + 1),
                _ => unreachable!(),
            },
            (0, -1) => match current {
                '|' => (pos.0 - 1, pos.1),
                '7' => (pos.0, pos.1 - 1),
                'F' => (pos.0, pos.1 + 1),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    let input = include_str!("input");
    let mut prev_pos = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| l.chars().position(|c| c == 'S').map(|p| (i as i64, p as i64)))
        .next()
        .unwrap();

    let mut count = 0;
    let mut pos = prev_pos;

    loop { 
        count += 1;
        let current = input.lines().nth(pos.0 as usize).unwrap().chars().nth(pos.1 as usize).unwrap();

        if current == 'S' {
            break;
        }

        let next_pos = do_move(pos, (pos.0 - prev_pos.0, pos.1 - prev_pos.1), current);

        prev_pos = pos;
        pos = next_pos;
    }

    println!("{}", count / 2)
}

fn main() {
    a();
}
