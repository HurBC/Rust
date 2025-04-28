#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct PointWithValue {
    value: f32,
    point: Point
}

impl Point {
    pub fn is_x_or_y_cell(&self, other: &Self) -> bool {
        ((other.x == self.x - 1.0 || other.x == self.x + 1.0) && other.y == self.y)
            || ((other.y == self.y - 1.0 || other.y == self.y + 1.0) && self.x == other.x)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Player {
    value: f32,
    point: Point,
}

const CELLS_CHECKOUT_SIZE: i32 = 3;

type MapType = [[f32; 8]; 8];

fn print_map(map: MapType) {
    for (index, row) in map.iter().enumerate() {
        println!(
            "\t{:?}{}",
            row,
            if index != map.len() - 1 { "," } else { "" }
        );
    }
}

fn check_cell(point_a: &Point, point_b: &Point, weithg: f32, map: &mut MapType) -> Option<f32> {

    if let Some(row) = map.get_mut(point_a.x as usize) {
        if let Some(col) = row.get_mut(point_a.y as usize) {
            if *col != 0.0 {
                *col = -1.0;

                return Some(*col);
            } else {
                let side_1 = point_b.y - point_a.y;
                let side_2 = point_b.x - point_a.x;

                let powered_side_1 = side_1.powf(2.0);
                let powered_side_2 = side_2.powf(2.0);

                *col = (powered_side_1 + powered_side_2) as f32 * weithg;

                return Some(*col);
            }
        }
    }

    None
}

fn check_cells(point_a: &Point, point_b: &Point, map: &mut MapType) {
    // Left Corner Point
    let mut index_point = Point {
        x: point_a.x - 1.0,
        y: point_a.y - 1.0,
    };

    let mut points = Vec::<PointWithValue>::new();

    for _ in 0..CELLS_CHECKOUT_SIZE {
        for _ in 0..CELLS_CHECKOUT_SIZE {
            if index_point != *point_a {
                let weithg = if point_a.is_x_or_y_cell(&index_point) {
                    1.0
                } else {
                    2.0
                } ;

                let cost = check_cell(&index_point, point_b, weithg, map);

                if let Some(cost) = cost {
                    points.push(PointWithValue { value: cost, point: index_point.clone() });
                }
            }

            index_point.y += 1.0;
        }

        index_point.x += 1.0;
        index_point.y = point_a.y - 1.0;
    }

    println!("{:?}", points);

    let mut min = 0.0;

    for p in points.iter() {
        
    }

}

fn move_cell() {
    
}

fn main() {
    let mut small_map: MapType = [[0.0; 8]; 8];

    let player_x = Player {
        value: 2.0,
        point: Point { x: 1.0, y: 1.0 },
    };

    let player_y = Player {
        value: 2.0,
        point: Point { x: 3.0, y: 5.0 },
    };

    small_map[player_x.point.x as usize][player_x.point.y as usize] = player_x.value;
    small_map[player_y.point.x as usize][player_y.point.y as usize] = player_y.value;

    small_map[1][2] = 1.0;

    check_cells(&player_x.point, &player_y.point, &mut small_map);

    print_map(small_map);
}
