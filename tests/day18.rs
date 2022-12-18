use itertools::Itertools;
use std::{collections::HashSet, ops::Add};

type Delta = (i32, i32, i32);

const INPUT: &str = include_str!("res/18.txt");
const DIRECTIONS: [Delta; 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point(i32, i32, i32);

impl Add<Delta> for &Point {
    type Output = Point;

    fn add(self, rhs: Delta) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

fn parse(input: &str) -> Point {
    let coords = input
        .split(",")
        .map(str::parse)
        .filter_map(Result::ok)
        .collect_vec();
    Point(coords[0], coords[1], coords[2])
}

fn exposed(point: &Point, shape: &HashSet<Point>) -> usize {
    DIRECTIONS
        .iter()
        .map(|direction| point + *direction)
        .filter(|p| !shape.contains(p))
        .count()
}

fn surface_area(shape: &HashSet<Point>) -> usize {
    shape.iter().map(|point| exposed(point, shape)).sum()
}

#[test]
fn part1() {
    let shape = INPUT.lines().map(parse).collect();
    println!("Day 18, part 1: {}", surface_area(&shape));
}

#[derive(Debug)]
struct Bounds {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl Bounds {
    fn new(shape: &HashSet<Point>) -> Self {
        Self {
            x_min: shape.iter().map(|p| p.0).min().unwrap() - 1,
            x_max: shape.iter().map(|p| p.0).max().unwrap() + 1,
            y_min: shape.iter().map(|p| p.1).min().unwrap() - 1,
            y_max: shape.iter().map(|p| p.1).max().unwrap() + 1,
            z_min: shape.iter().map(|p| p.2).min().unwrap() - 1,
            z_max: shape.iter().map(|p| p.2).max().unwrap() + 1,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        if !(self.x_min <= point.0 && point.0 <= self.x_max) {
            return false;
        }
        if !(self.y_min <= point.1 && point.1 <= self.y_max) {
            return false;
        }
        if !(self.z_min <= point.2 && point.2 <= self.z_max) {
            return false;
        }
        true
    }

    fn exterior_surface_area(&self) -> i32 {
        let delta_x = (self.x_max - self.x_min) + 1;
        let delta_y = (self.y_max - self.y_min) + 1;
        let delta_z = (self.z_max - self.z_min) + 1;

        (2 * delta_x * delta_y) + (2 * delta_x * delta_z) + (2 * delta_y * delta_z)
    }
}

fn fill(point: Point, shape: &HashSet<Point>, complement: &mut HashSet<Point>, bounds: &Bounds) {
    if shape.contains(&point) {
        return;
    }
    let candidates = DIRECTIONS
        .iter()
        .map(|direction| &point + *direction)
        .filter(|point| bounds.contains(point))
        .filter(|point| !shape.contains(point))
        .filter(|point| !complement.contains(point))
        .collect_vec();
    complement.insert(point);
    for candidate in candidates {
        fill(candidate, shape, complement, bounds);
    }
}

#[test]
fn part2() {
    // we need to find the bounds of the shape and then do some ray marching
    let shape: HashSet<Point> = INPUT.lines().map(parse).collect();
    let bounds = Bounds::new(&shape);

    // We're going to start "filling" this bounded cube of space with a
    // complementary set. When we've finished "inflating" this balloon, we'll
    // know that any remaining non-occupied spaces within the bounds which are
    // not part of the complementary set are air pockets.
    let mut complement = HashSet::<Point>::new();
    let start = Point(bounds.x_max, bounds.y_max, bounds.z_max);

    // this needs to be run in release mode otherwise the stack overflows...
    fill(start, &shape, &mut complement, &bounds);

    // We can use the surface area of the complement to get the exterior surface
    // area of our main shape, but we need to subtract the exterior surface area
    // of the *complement* first. Thankfully, since it's guaranteed to be some
    // kind of cuboid we actually have an explicit equation for it.s
    let ans = surface_area(&complement) - bounds.exterior_surface_area() as usize;
    println!("Day 18, part 2: {ans}");
}
