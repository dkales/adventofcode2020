#![feature(str_split_once)]
use std::{collections::HashMap, fmt::Display, path::PathBuf, vec};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day20")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}
#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    data: Vec<Vec<char>>,
    dimension: usize,
}
impl Tile {
    fn from_str(input: &str) -> Self {
        let (id, data) = input.split_once("\r\n").expect("valid input");
        let id = id
            .trim_start_matches("Tile ")
            .trim_end_matches(":")
            .parse::<usize>()
            .expect("valid id");
        let data: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
        Tile {
            id,
            dimension: data.len(),
            data,
        }
    }
    fn north(&self) -> usize {
        self.data[0]
            .iter()
            .fold(0usize, |acc, &x| acc * 2 + if x == '#' { 1 } else { 0 })
    }
    fn south(&self) -> usize {
        self.data[self.data.len() - 1]
            .iter()
            .fold(0usize, |acc, &x| acc * 2 + if x == '#' { 1 } else { 0 })
    }
    fn east(&self) -> usize {
        self.data.iter().fold(0usize, |acc, x| {
            acc * 2 + if x[x.len() - 1] == '#' { 1 } else { 0 }
        })
    }
    fn west(&self) -> usize {
        self.data
            .iter()
            .fold(0usize, |acc, x| acc * 2 + if x[0] == '#' { 1 } else { 0 })
    }

    fn edges(&self) -> [usize; 8] {
        let north_mir = self.data[0]
            .iter()
            .rev()
            .fold(0usize, |acc, &x| acc * 2 + if x == '#' { 1 } else { 0 });
        let south_mir = self.data[self.data.len() - 1]
            .iter()
            .rev()
            .fold(0usize, |acc, &x| acc * 2 + if x == '#' { 1 } else { 0 });
        let west_mir = self
            .data
            .iter()
            .rev()
            .fold(0usize, |acc, x| acc * 2 + if x[0] == '#' { 1 } else { 0 });
        let east_mir = self.data.iter().rev().fold(0usize, |acc, x| {
            acc * 2 + if x[x.len() - 1] == '#' { 1 } else { 0 }
        });
        [
            self.north(),
            self.east(),
            self.south(),
            self.west(),
            north_mir,
            east_mir,
            south_mir,
            west_mir,
        ]
    }
    fn rotate(&self) -> Tile {
        let dim = self.data.len();
        let mut data = vec![vec!['x'; dim]; dim];
        for i in 0..dim {
            for j in 0..dim {
                data[i][j] = self.data[j][dim - 1 - i];
            }
        }
        Tile {
            id: self.id,
            data,
            dimension: self.dimension,
        }
    }
    fn mirror_horizontal(&self) -> Tile {
        Tile {
            id: self.id,
            data: self
                .data
                .iter()
                .map(|x| x.iter().rev().copied().collect())
                .collect(),
            dimension: self.dimension,
        }
    }
    fn mirror_vertical(&self) -> Tile {
        Tile {
            id: self.id,
            data: self.data.iter().rev().cloned().collect(),
            dimension: self.dimension,
        }
    }
    fn variants(&self) -> Vec<Tile> {
        let mut ret = Vec::new();
        ret.push(self.clone());
        ret.push(ret.last().unwrap().rotate());
        ret.push(ret.last().unwrap().rotate());
        ret.push(ret.last().unwrap().rotate());
        ret.push(self.mirror_horizontal());
        ret.push(ret.last().unwrap().rotate());
        ret.push(ret.last().unwrap().rotate());
        ret.push(ret.last().unwrap().rotate());
        ret
    }
    fn trimmed(&self) -> Tile {
        Tile {
            id: self.id,
            data: self
                .data
                .iter()
                .skip(1)
                .take(self.dimension - 2)
                .map(|x| x.iter().skip(1).take(self.dimension - 2).copied().collect())
                .collect(),
            dimension: self.dimension - 2,
        }
    }

    fn find_and_replace_monster(&self) -> Option<Tile> {
        let mut monster_count = 0;
        let mut copy = self.clone();
        for i in 0..self.dimension - 2 {
            for j in 18..self.dimension {
                if self.data[i].get(j) == Some(&'#')
                    && self.data[i + 1].get(j) == Some(&'#')
                    && self.data[i + 1].get(j - 1) == Some(&'#')
                    && self.data[i + 1].get(j + 1) == Some(&'#')
                    && self.data[i + 2].get(j - 2) == Some(&'#')
                    && self.data[i + 2].get(j - 5) == Some(&'#')
                    && self.data[i + 1].get(j - 6) == Some(&'#')
                    && self.data[i + 1].get(j - 7) == Some(&'#')
                    && self.data[i + 2].get(j - 8) == Some(&'#')
                    && self.data[i + 2].get(j - 11) == Some(&'#')
                    && self.data[i + 1].get(j - 12) == Some(&'#')
                    && self.data[i + 1].get(j - 13) == Some(&'#')
                    && self.data[i + 2].get(j - 14) == Some(&'#')
                    && self.data[i + 2].get(j - 17) == Some(&'#')
                    && self.data[i + 1].get(j - 18) == Some(&'#')
                {
                    // found monster, replace it
                    monster_count += 1;
                    copy.data[i + 0][j + 0] = 'O';
                    copy.data[i + 1][j + 0] = 'O';
                    copy.data[i + 1][j - 1] = 'O';
                    copy.data[i + 1][j + 1] = 'O';
                    copy.data[i + 2][j - 2] = 'O';
                    copy.data[i + 2][j - 5] = 'O';
                    copy.data[i + 1][j - 6] = 'O';
                    copy.data[i + 1][j - 7] = 'O';
                    copy.data[i + 2][j - 8] = 'O';
                    copy.data[i + 2][j - 11] = 'O';
                    copy.data[i + 1][j - 12] = 'O';
                    copy.data[i + 1][j - 13] = 'O';
                    copy.data[i + 2][j - 14] = 'O';
                    copy.data[i + 2][j - 17] = 'O';
                    copy.data[i + 1][j - 18] = 'O';
                }
            }
        }

        if monster_count > 0 {
            Some(copy)
        } else {
            None
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|x| x.iter().collect())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        self.id == other.id
    }
}

fn part1(input: &[Tile]) -> Vec<Vec<char>> {
    let dimensions = (input.len() as f64).sqrt() as usize;
    assert!(dimensions * dimensions == input.len());
    let mut edges = HashMap::new();
    for a in input
        .iter()
        .map(|x| x.edges().iter().copied().collect::<Vec<_>>())
        .flatten()
    {
        edges
            .entry(a)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1usize);
    }
    let corners: Vec<_> = input
        .iter()
        .filter(|&x| {
            x.edges()
                .iter()
                .filter(|&edge| edges.get(edge) == Some(&2))
                .count()
                == 4
        })
        .collect();
    let mut edge_pieces: Vec<_> = input
        .iter()
        .filter(|&x| {
            x.edges()
                .iter()
                .filter(|&edge| edges.get(edge) == Some(&2))
                .count()
                == 6
        })
        .collect();
    assert!(corners.len() == 4);
    assert!(edge_pieces.len() == (dimensions - 2) * 4);
    println!("Part1: {}", corners.iter().map(|x| x.id).product::<usize>());
    //println!("{:?}", corners);
    let mut finished = HashMap::new();
    // find correct first corner
    let a = corners[0]
        .variants()
        .into_iter()
        .filter(|x| edges.get(&x.north()) == Some(&1) && edges.get(&x.west()) == Some(&1))
        .collect::<Vec<_>>();
    assert!(a.len() == 2); //first has 2 possibilites, rest should be fixed then
    finished.insert((0, 0), a[0].clone());
    for i in 1..dimensions - 1 {
        let a: Vec<_> = edge_pieces
            .iter()
            .map(|x| x.variants())
            .flatten()
            .filter(|x| {
                edges.get(&x.north()) == Some(&1)
                    && x.west() == finished.get(&(0, i - 1)).unwrap().east()
            })
            .collect();
        assert!(a.len() == 1);
        finished.insert((0, i), a[0].clone());
        edge_pieces.retain(|x| *x != &a[0]);
    }
    // find correct second corner
    let a = corners
        .iter()
        .map(|x| x.variants())
        .flatten()
        .filter(|x| {
            edges.get(&x.north()) == Some(&1)
                && edges.get(&x.east()) == Some(&1)
                && x.west() == finished.get(&(0, dimensions - 2)).unwrap().east()
        })
        .collect::<Vec<_>>();
    assert!(a.len() == 1);
    finished.insert((0, dimensions - 1), a[0].clone());

    for i in 1..dimensions - 1 {
        let a: Vec<_> = edge_pieces
            .iter()
            .map(|x| x.variants())
            .flatten()
            .filter(|x| {
                edges.get(&x.west()) == Some(&1)
                    && x.north() == finished.get(&(i - 1, 0)).unwrap().south()
            })
            .collect();
        assert!(a.len() == 1);
        finished.insert((i, 0), a[0].clone());
        edge_pieces.retain(|x| *x != &a[0]);
    }
    // find correct third corner
    let a = corners
        .iter()
        .map(|x| x.variants())
        .flatten()
        .filter(|x| {
            edges.get(&x.south()) == Some(&1)
                && edges.get(&x.west()) == Some(&1)
                && x.north() == finished.get(&(dimensions - 2, 0)).unwrap().south()
        })
        .collect::<Vec<_>>();
    assert!(a.len() == 1);
    finished.insert((dimensions - 1, 0), a[0].clone());
    // fill up middle
    for i in 1..dimensions {
        for j in 1..dimensions {
            let a: Vec<_> = input
                .iter()
                .map(|x| x.variants())
                .flatten()
                .filter(|x| {
                    x.west() == finished.get(&(i, j - 1)).unwrap().east()
                        && x.north() == finished.get(&(i - 1, j)).unwrap().south()
                })
                .collect();
            assert!(a.len() == 1);
            finished.insert((i, j), a[0].clone());
        }
    }
    let small_dim = a[0].dimension - 2;
    let mut ret = vec![vec!['x'; dimensions * small_dim]; dimensions * small_dim];
    for i in 0..dimensions {
        for j in 0..dimensions {
            let pic = finished.get_mut(&(i, j)).unwrap().trimmed().data;
            for x in 0..small_dim {
                for y in 0..small_dim {
                    ret[i * small_dim + x][j * small_dim + y] = pic[x][y];
                }
            }
        }
    }

    ret
}
fn part2(input: &Vec<Vec<char>>) {
    let tile = Tile {
        id: 0,
        data: input.clone(),
        dimension: input.len(),
    };
    let roughness = tile
        .variants()
        .iter()
        .filter_map(|x| x.find_and_replace_monster())
        .next()
        .expect("have one tile with monsters")
        .data
        .iter()
        .flatten()
        .filter(|&x| x == &'#')
        .count();
    println!("{}", roughness);
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    let tiles: Vec<_> = content.split("\r\n\r\n").map(Tile::from_str).collect();

    let pic = part1(&tiles);

    part2(&pic);
}
