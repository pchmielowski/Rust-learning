#![deny(warnings)]

pub mod lib {
    use std::cmp::Ordering;
    use std::f32::MIN;

    #[derive(Clone, Copy)]
    pub struct Platform {
        pub    x_from: Meters,
        pub    x_to: Meters,
        pub y: Meters,
    }

    pub type CoinValue = u8;

    pub struct Coin {
        pub x: Meters,
        pub y: Meters,
        #[allow(dead_code)] // TODO!
        pub value: CoinValue,
    }

    pub struct Board {
        pub platforms: Vec<Platform>,
        pub coins: Vec<Coin>,
    }

    impl Platform {
        pub fn width(self) -> Meters {
            self.x_to - self.x_from
        }
    }

    impl Default for Board {
        fn default() -> Self {
            let size = 20;
            let to_platform = |n: i32| Platform {
                x_from: (n * size - size / 2) as f32,
                x_to: (n * size + size / 2) as f32,
                y: (-n) as f32,
            };
            let to_coins = |n: i32| Coin {
                x: (n * size) as f32,
                y: (-n + 1) as f32,
                value: n as u8,
            };
            Board {
                platforms: (0..10).map(to_platform).collect(),
                coins: (0..10).map(to_coins).collect(),
            }
        }
    }

    #[derive(PartialEq)]
    pub enum Direction {
        Forward,
        Backward,
    }

    impl Default for Direction {
        fn default() -> Self { Direction::Forward }
    }

    impl Direction {
        pub fn speed(&self) -> Meters {
            let speed: MetersPerSecond = 6.0;
            match self {
                Direction::Forward => speed,
                Direction::Backward => -speed,
            }
        }
    }

    pub type Meters = f32;
    pub type MetersPerSecond = f32;

    pub type Millis = i32;
    pub type Seconds = f32;

    pub struct State {
        pub direction: Direction,
        pub is_moving: bool,
        pub is_on_ground: bool,
        pub x: Meters,
        pub y: Meters,
        pub speed_y: MetersPerSecond,
        pub board: Board,
    }

    impl Default for State {
        fn default() -> Self {
            State {
                x: 0.0,
                y: 10.0,
                speed_y: 0.0,
                direction: Direction::default(),
                is_moving: false,
                is_on_ground: false,
                board: Board::default(),
            }
        }
    }

    pub fn millis_to_seconds(millis: Millis) -> Seconds {
        millis as Seconds / 1000.0
    }

    impl State {
        pub fn go_in_direction(self, direction: Direction) -> Self {
            if self.is_on_ground {
                State {
                    direction,
                    is_moving: true,
                    ..self
                }
            } else {
                self
            }
        }
        pub fn go_forward(self) -> Self {
            self.go_in_direction(Direction::Forward)
        }

        pub fn go_backward(self) -> Self {
            self.go_in_direction(Direction::Backward)
        }

        pub fn stop(self) -> Self {
            if self.is_on_ground {
                State {
                    is_moving: false,
                    ..self
                }
            } else {
                self
            }
        }

        pub fn jump(self) -> Self {
            State {
                speed_y: 7.0,
                ..self
            }
        }

        pub fn update(self, time_delta: Millis) -> Self {
            let seconds = millis_to_seconds(time_delta);
            let x_delta = if self.is_moving {
                seconds * self.direction.speed()
            } else {
                0.0
            };
            let g = 9.81; // m/s^2
            let platform_below = self.platform_below();
            let platform_on_left = self.platform_on_left()
                .map_or(MIN, |platform| platform.x_to);
            let x = (self.x + x_delta).max(platform_on_left);
            let y = (self.y + self.speed_y * seconds).max(platform_below);
            State {
                x,
                y,
                is_on_ground: y == platform_below,
                speed_y: self.speed_y - g * seconds,
                ..self
            }
        }

        pub fn platform_on_left(&self) -> Option<Platform> {
            let mut vec: Vec<&Platform> = self.board.platforms.iter()
                .filter(|platform| platform.y >= self.y)
                .filter(|platform| platform.x_to <= self.x)
                .collect();
            // TODO: Cleanup usage of & and * because I don't think it should be written this way.
            vec.sort_by(|a, b| (**b).x_to.partial_cmp(&(**a).x_to).unwrap_or(Ordering::Equal));
            vec.first().map(|platform| **platform)
        }

        pub fn platform_below(&self) -> Meters {
            let mut vec: Vec<Meters> = self.board.platforms.iter()
                .filter(|platform| platform.x_from <= self.x && platform.x_to >= self.x)
                .filter(|platform| platform.y <= self.y)
                .map(|platform| platform.y)
                .collect();
            vec.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
            *vec.first().ok_or("No ground below!").unwrap()
        }
    }

    #[test]
    fn finds_platform_below() {
        let x = 5.0;
        let state = State {
            x,
            y: 2.0,
            speed_y: 0.0,
            direction: Direction::default(),
            is_moving: false,
            is_on_ground: false,
            board: Board {
                platforms: vec![
                    Platform { x_from: x - 1.0, x_to: x + 1.0, y: 100.0 },
                    Platform { x_from: x - 1.0, x_to: x + 1.0, y: 1.0 },
                    Platform { x_from: x - 2.0, x_to: x + 0.5, y: 1.5 },
                    Platform { x_from: x + 2.0, x_to: x + 7.5, y: 3.0 },
                ],
                coins: vec![]
            },
        };
        assert_eq!(state.platform_below(), 1.5);
    }

    pub trait ToPixels {
        //noinspection RsSelfConvention
        fn to_pixels(self) -> i32;
    }

    impl ToPixels for Meters {
        fn to_pixels(self) -> i32 {
            (self * 60.0) as i32
        }
    }
}