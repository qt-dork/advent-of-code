use itertools::Itertools;


fn main() {
    let input = include_str!("day8.txt");
    let res1 = process_part1(input);
    let res2 = process_part2(input);
    println!("{}", res1);
    println!("{}", res2);
}

struct FakeGrid {
    grid: Vec<usize>,
    index: Vec<usize>,
    width: usize,
}

impl FakeGrid {
    fn get_row(&self, x: usize) -> Vec<(usize, usize)> {
        dbg!(x);
        let items = &self.grid[(x*self.width)..(x*self.width+self.width)];
        let indexes = &self.index[(x*self.width)..(x*self.width+self.width)];
        items.iter().zip(indexes.iter()).map(|(x, y)| (*x, *y)).collect_vec()
    }
    fn get_column(&self, y: usize) -> Vec<(usize, usize)> {
        let items = self.grid.chunks(self.width).map(|a| a[y]).collect::<Vec<_>>();
        let indexes = self.index.chunks(self.width).map(|a| a[y]).collect::<Vec<_>>();
        items.iter().zip(indexes.iter()).map(|(x, y)| (*x, *y)).collect_vec()
    }
}

// bandaid
fn vecs_to_slices<T>(vecs: &[Vec<T>]) -> Vec<&[T]> {
    vecs.iter().map(Vec::as_slice).collect()
}

fn visible(grid: FakeGrid) -> usize {
    let visible: Vec<_> = vec![];
    // only works if it's a grid lol
    let rows = (0..grid.width).into_iter().map(|x| grid.get_row(x)).collect_vec();
    dbg!(rows.clone());
    let rows = vecs_to_slices(&rows);
    let columns = (0..grid.width).into_iter().map(|y| grid.get_column(y)).collect_vec();
    let columns = vecs_to_slices(&columns);

    let visible_rows: Vec<&(usize, usize)> = rows.iter().flat_map(|&row| row.iter().filter(|&(x, index)| {
        let mut split = row.split(|(y, i)| y == x && i == index);
        let left = split.next().unwrap().iter().max().unwrap_or(&(0, 0));
        let right = split.next().unwrap().iter().max().unwrap_or(&(0, 0));
        let mut r = x > &left.0 || x > &right.0;
        if left == &(0,0) || right == &(0,0) {
            r = true;
        }
        dbg!(r);
        r
    })).collect_vec();
    let visible_columns: Vec<&(usize, usize)> = columns.iter().flat_map(|&column| column.iter().filter(|&(y, index)| {
        let mut split = column.split(|(x, i)| y == x && i == index);
        let left = split.next().unwrap().iter().max().unwrap_or(&(0, 0));
        let right = split.next().unwrap().iter().max().unwrap_or(&(0, 0));
        let mut r = y > &left.0 || y > &right.0;
        if left == &(0,0) || right == &(0,0) {
            r = true;
        }
        dbg!(r);
        r
    })).collect_vec();

    let visible = visible.iter()
        .chain(visible_rows.iter().copied())
        .chain(visible_columns.iter().copied())
        .map(|(x, index)| (index, x))
        .collect_vec();
    dbg!(visible.clone());
    let visible = visible.iter().sorted().dedup().collect_vec();
    dbg!(visible.clone());
    visible.iter().count()

}

fn process_part1(i: &str) -> usize {
    let mut lines = i.lines().peekable();
    let width = lines.peek().unwrap().len();
    dbg!(width);
    let grid: Vec<usize> = lines.flat_map(|line| line.chars().flat_map(|c| c.to_digit(10))).map(|x| x as usize).collect();
    let len = grid.len();
    let index = (0..len).collect_vec();
    dbg!(index.clone());
    

    let fake_grid = FakeGrid {
        grid,
        index,
        width,
    };

    visible(fake_grid)
}

fn process_part2(i: &str) -> usize {
    let mut lines = i.lines().peekable();
    let width = lines.peek().unwrap().len();
    dbg!(width);
    let grid: Vec<usize> = lines.flat_map(|line| line.chars().flat_map(|c| c.to_digit(10))).map(|x| x as usize).collect();
    let len = grid.len();
    let index = (0..len).collect_vec();
    dbg!(index.clone());
    

    let grid = FakeGrid {
        grid,
        index,
        width,
    };

    let visible: Vec<_> = vec![];
    // only works if it's a grid lol
    let rows = (0..grid.width).into_iter().map(|x| grid.get_row(x)).collect_vec();
    dbg!(rows.clone());
    let rows = vecs_to_slices(&rows);
    let columns = (0..grid.width).into_iter().map(|y| grid.get_column(y)).collect_vec();
    let columns = vecs_to_slices(&columns);

    let visible_rows: Vec<(usize, usize, usize)> = rows.iter().flat_map(|&row| row.iter().filter_map(|(x, index)| {
        let mut split = row.split(|(y, i)| y == x && i == index);
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        let is_visible = {
            let (left_max, _) = left.iter().max().unwrap_or(&(0,0));
            let (right_max, _) = right.iter().max().unwrap_or(&(0,0));
            x > left_max || x > right_max
        };
        if is_visible {
            let score = {
                let (l, _) = left.iter().rfold((0, false), |(accum, breaker), (item, _)| {
                    if breaker {
                        return (accum, breaker);
                    }
                    let v = x > item;
                    (
                        accum + 1,
                        !v
                    )
                });
                let (r, _) = right.iter().fold((0, false), |(accum, breaker), (item, _)| {
                    if breaker {
                        return (accum, breaker);
                    }
                    let v = x > item;
                    (
                        accum + 1,
                        !v
                    )
                });
                l * r
            };
            return Some((*x, *index, score));
        } else {
            return None;
        }
    })).collect_vec();
    let visible_columns: Vec<(usize, usize, usize)> = columns.iter().flat_map(|&column| column.iter().filter_map(|&(y, index)| {
        let mut split = column.split(|(x, i)| &y == x && i == &index);
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        let is_visible = {
            let (left_max, _) = left.iter().max().unwrap_or(&(0,0));
            let (right_max, _) = right.iter().max().unwrap_or(&(0,0));
            &y > left_max || &y > right_max
        };
        if is_visible {
            let score = {
                let (l, _) = left.iter().rfold((0, false), |(accum, breaker), (item, _)| {
                    if breaker {
                        return (accum, breaker);
                    }
                    let v = &y > item;
                    (
                        accum + 1,
                        !v
                    )
                });
                let (r, _) = right.iter().fold((0, false), |(accum, breaker), (item, _)| {
                    if breaker {
                        return (accum, breaker);
                    }
                    let v = &y > item;
                    (
                        accum + 1,
                        !v
                    )
                });
                l * r
            };
            return Some((y, index, score));
        } else {
            return None;
        }
    })).collect_vec();

    visible.into_iter()
        .chain(visible_rows.iter())
        .chain(visible_columns.iter())
        .sorted()
        .copied()
        .coalesce(|x, y| {
            if x.1 == y.1 {
                dbg!((x, y));
                Ok((x.0, x.1 , x.2*y.2))
            } else {
                Err((x, y))
            }
        })
        .reduce(|accum, item| if accum.2 > item.2 { dbg!(accum);accum } else { item })
        .unwrap_or((0,0,0))
        .2
}

#[cfg(test)]
#[test]
fn part1_works() {
    let input = include_str!("day8_test.txt");
    assert_eq!(process_part1(input), 21);
}

#[cfg(test)]
#[test]
fn part2_works() {
    let input = include_str!("day8_test.txt");
    assert_eq!(process_part2(input), 8);
}