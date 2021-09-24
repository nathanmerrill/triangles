use std::{collections::HashMap, fmt::Display};

use draw::{Canvas, Drawing, Shape};

fn main() {
    let grid_size = 12;
    let triangles = find_triangles(grid_size);

    println!("Triangles found:");
    for size in 2 .. grid_size+1 {
        let by_size = &triangles[&size];
        println!("Size {}. Count {}.", size, by_size.len());
        //for ((x1, y1), (x2, y2), (x3, y3)) in by_size.values() {
        //    println!("({}, {}), ({}, {}), ({}, {})", x1, y1, x2, y2, x3, y3);
        //}
    }
}
/*
fn draw_triangles(triangles: HashMap<u8, HashMap<(Slope, Slope, Slope), ((u8, u8), (u8, u8), (u8, u8))>>) {
    let triangle_count: usize = triangles.values().map(|a| a.len()).sum();
    let box_size: usize = triangles.len() + 2;
    let density = 5;

    let rows = (triangle_count as f64).sqrt().ceil() as usize;
    let width = box_size*density*rows;
    let mut canvas = Canvas::new(width, width);
    for line in 0 .. rows {
        Drawing::new().with_shape(Shape::Line{
            start: Point2 { x: line * density, y: 0 },
            points: 
        })
    }
}
*/
fn find_triangles(grid_size: u8) -> HashMap<u8, HashMap<(Slope, Slope, Slope), ((u8, u8), (u8, u8), (u8, u8))>> {
    let mut triangles = HashMap::new();

    for width in 1 .. grid_size {
        triangles.insert(width+1, HashMap::new());
    }
    let x1 = 0;
    let y1 = 0;
    for x2 in 0 .. grid_size {
        for y2 in 0 .. grid_size {
            let slope1 = slope(x1, y1, x2, y2);
            if valid_line(slope1) {
                for x3 in 0 .. grid_size {
                    for y3 in 0 .. grid_size {
                        let slope2 = slope(x1, y1, x3, y3);
                        let slope3 = slope(x2, y2, x3, y3);
                        if valid_line(slope2){
                            if valid_line(slope3) {
                                let slopes = {
                                    if slope1 < slope2 {
                                        if slope2 < slope3 {
                                            (slope1, slope2, slope3)
                                        } else if slope3 < slope1 {
                                            (slope3, slope1, slope2)
                                        } else {
                                            (slope1, slope3, slope2)
                                        }
                                    } else {
                                        if slope1 < slope3 {
                                            (slope2, slope1, slope3)
                                        } else if slope3 < slope2 {
                                            (slope3, slope2, slope1)
                                        } else {
                                            (slope2, slope3, slope1)
                                        }
                                    }
                                };                                      
                                
                                let required_grid_size = slopes.2.large+1;

                                triangles.get_mut(&required_grid_size).unwrap().entry(slopes).or_insert(((x1, y1), (x2, y2), (x3, y3)));
                            }
                        }
                    }
                }
            }
        }
    }

    triangles
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Slope {
    large: u8,
    small: u8
}

impl Display for Slope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.large, self.small)
    }
}

fn valid_line(slope: Slope) -> bool {
    if slope.small == 0 {
        slope.large == 1
    } else {
        num::integer::gcd(slope.small, slope.large) == 1
    }
}

fn slope(x1: u8, y1: u8, x2: u8, y2: u8) -> Slope {
    let x = if x1 > x2 {
        x1 - x2
    } else {
        x2 - x1
    };

    let y = if y1 > y2 {
        y1 - y2
    } else {
        y2 - y1
    };

    if x > y {
        Slope{small: y, large: x}
    } else {
        Slope{small: x, large: y}
    }
}
