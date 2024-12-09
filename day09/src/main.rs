fn solve1(input: &str) -> usize {
    let lines = input.lines();
    let mut tot_checksum = 0;
    for line in lines {
        let v = line
            .chars()
            .filter(char::is_ascii)
            .map(|c| c as u8 - b'0')
            .collect::<Vec<_>>();
        let mut files = v
            .as_slice()
            .chunks(2)
            .map(|w| {
                if w.len() == 2 {
                    (w[0], w[1])
                } else {
                    (w[0], 0)
                }
            })
            .collect::<Vec<_>>();
        let mut count = 0;
        let mut checksum = 0;
        let mut i = 0;
        while i < files.len() {
            let (mem, mut frag) = files[i];
            for _ in 0..mem {
                checksum += i * count;
                count += 1;
            }
            while frag > 0 && i != files.len() - 1 {
                let n = files.len();
                let Some((ref mut end_mem, _)) = files.last_mut() else {
                    break;
                };
                let num_filled = std::cmp::min(*end_mem, frag);
                *end_mem -= num_filled;
                frag -= num_filled;
                for _ in 0..num_filled {
                    let id = n - 1;
                    checksum += id * count;
                    count += 1;
                }
                if *end_mem == 0 {
                    files.pop();
                }
            }
            i += 1;
        }
        tot_checksum += checksum;
    }
    tot_checksum
}

fn solve2(input: &str) -> usize {
    let lines = input.lines();
    let mut tot_checksum = 0;
    for line in lines {
        let v = line
            .chars()
            .filter(char::is_ascii)
            .map(|c| (c as u8 - b'0') as usize)
            .collect::<Vec<_>>();
        let mut count = 0;
        let mut files: Vec<(usize, usize, usize, usize)> = v
            .as_slice()
            .chunks(2)
            .map(|w| {
                if w.len() == 2 {
                    let tmp = count;
                    count += w[0] + w[1];
                    (tmp, w[0], 0, w[1])
                } else {
                    let tmp = count;
                    count += w[0];
                    (tmp, w[0], 0, 0)
                }
            })
            .collect();

        let mut checksum = 0;
        let mut i = files.len();
        'main_loop: while i > 0 {
            i -= 1;
            let (loc0, mem0, _, _) = *files.get(i).unwrap();

            for j in 0..i {
                let (loc, mem, filled, left) = files.get_mut(j).unwrap();
                if *left >= mem0 {
                    for l in 0..mem0 {
                        let ll = *loc + *mem + *filled + l;
                        checksum += i * ll;
                    }
                    *filled += mem0;
                    *left -= mem0;
                    continue 'main_loop;
                }
            }

            for l in loc0..(loc0 + mem0) {
                checksum += i * l;
            }
        }
        tot_checksum += checksum;
    }
    tot_checksum
}

fn main() {
    let input = include_str!("../input.txt");
    let fs = [solve1, solve2];

    solver::solver(fs, input);
}
