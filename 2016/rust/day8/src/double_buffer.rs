use regex::Regex;

fn index(width: usize, x: usize, y: usize) -> usize {
    x + (y * width)
}

fn count(screen: &[bool]) -> usize {
    screen.iter().filter(|x| **x).count()
}

fn fill(width: usize, _height: usize, fill_width: usize, fill_height: usize, screen: &mut [bool]) {
    for y in 0..fill_height {
        for x in 0..fill_width {
            screen[index(width, x, y)] = true;
        }
    }
}

fn rotate_row(
    width: usize,
    _height: usize,
    y: usize,
    count: usize,
    source: &[bool],
    screen: &mut [bool],
) {
    for x in 0..width {
        let new_x = (x + count) % width;
        let new_pos = index(width, new_x, y);
        let old_pos = index(width, x, y);
        //println!("{} {} {} || {} {}", x, y, count, new_pos, old_pos);
        screen[new_pos] = source[old_pos];
        //show_screen(width, height, screen, source);
    }
}

fn rotate_column(
    width: usize,
    height: usize,
    x: usize,
    count: usize,
    source: &[bool],
    screen: &mut [bool],
) {
    for y in 0..height {
        let new_y = (y + count) % height;
        screen[index(width, x, new_y)] = source[index(width, x, y)];
    }
}

fn save(screen: &mut [bool], other: &[bool]) {
    screen.copy_from_slice(other);
}

fn show_screen(width: usize, height: usize, screen1: &[bool], screen2: &[bool]) {
    for _ in 0..width {
        print!("=");
    }
    print!("||");
    for _ in 0..width {
        print!("=");
    }

    println!();
    for y in 0..height {
        for x in 0..width {
            let symbol = match screen1[index(width, x, y)] {
                true => '#',
                false => '.',
            };
            print!("{symbol}");
        }
        print!("||");
        for x in 0..width {
            let symbol = match screen2[index(width, x, y)] {
                true => '#',
                false => '.',
            };
            print!("{symbol}");
        }
        println!();
    }
}

fn main() {
    let data = include_str!("../input");

    let rect_re = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let rotate_re = Regex::new(r"rotate (column|row) (?:x|y)=(\d+) by (\d+)").unwrap();

    let width = 50;
    let height = 6;
    let mut backend1 = vec![false; width * height];
    let mut backend2 = vec![false; width * height];

    let operations: Vec<_> = data.lines().collect();
    for (idx, op) in operations.into_iter().enumerate() {
        println!("{}: {:?}", idx, op);
        if let Some(captures) = rect_re.captures(op) {
            let fill_width = (&captures[1]).parse().unwrap();
            let fill_height = (&captures[2]).parse().unwrap();
            fill(width, height, fill_width, fill_height, &mut backend1);
        } else if let Some(captures) = rotate_re.captures(op) {
            let count: usize = (&captures[3]).parse().unwrap();

            match &captures[1] {
                "row" => {
                    let y: usize = (&captures[2]).parse().unwrap();
                    rotate_row(width, height, y, count, &backend2, &mut backend1);
                }
                "column" => {
                    let x: usize = (&captures[2]).parse().unwrap();
                    rotate_column(width, height, x, count, &backend2, &mut backend1);
                }
                _ => unreachable!(),
            }
        } else {
            panic!("No match {op}");
        }

        //show_screen(width, height, &backend1, &backend2);

        save(&mut backend2, &backend1);

        //show_screen(width, height, &backend1, &backend2);
        // if idx == 30 {
        //     break;
        // }
    }

    show_screen(width, height, &backend1, &backend2);
    println!("{}", count(&backend1));
}

#[test]
fn tests() {
    assert_eq!(index(50, 0, 0), 0);
    assert_eq!(index(50, 49, 0), 49);
    assert_eq!(index(50, 0, 1), 50);
    assert_eq!(index(50, 4, 1), 54);
}
