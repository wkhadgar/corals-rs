use std::collections::VecDeque;
use std::f32::consts::PI;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use crate::cells::Cell;
use crate::vectors::Vector2;

pub struct Organism {
    cells: VecDeque<Cell>,
    bounds: Vec<Point>,
    color: Color,
    spatiality: f64,
    bias: f64,
    noise: bool,
}

impl Organism {
    pub fn new(
        pos: Vector2,
        size: u32,
        radius: f64,
        spatiality: f64,
        bias: f64,
        noise: bool,
    ) -> Self {
        let mut cells = VecDeque::new();
        let angle = (2.0 * PI as f64 / size as f64);
        for i in 0..size {
            cells.push_back(Cell::new(
                i as u64,
                pos + Vector2::new(
                    ((angle * (i as f64)).cos() * radius),
                    ((angle * (i as f64)).sin() * radius),
                ),
            ))
        }

        Self {
            bounds: vec![],
            cells,
            color: Color::RGBA(104, 150, 102, 90),
            spatiality,
            bias,
            noise,
        }
    }

    pub fn expand(&mut self) {
        for i in 0..self.cells.len() {
            let mut force = Vector2::default();
            for c in &self.cells {
                if c.id != self.cells[i].id {
                    let dist = self.cells[i].position - c.position;
                    let size = dist.length_sqr();
                    if size > self.spatiality * 2.0 {
                        continue;
                    }
                    force += dist.mag(1.0 / size);
                }
            }

            let accel = force * (self.bias / self.cells[i].mass);
            self.cells[i].velocity += accel;
            self.cells[i].velocity = self.cells[i].velocity * 0.8;
        }

        for i in 0..self.cells.len() {
            self.cells[i].react();
        }

        self.bounds = self
            .cells
            .iter()
            .map(|c| c.pos().into())
            .collect::<Vec<Point>>();
    }
    pub fn fill_gaps(&mut self) {
        let noisiness = 0.3;
        let amount = self.cells.len() - 1;

        let mut offset = 0;
        for i in 0..amount {
            let pos_ref = self.cells[i + offset].pos();
            let dist = self.cells[i + offset + 1].pos() - pos_ref;
            if dist.length_sqr() > self.spatiality {
                self.cells.insert(
                    i + offset + 1,
                    Cell::new(
                        (amount + offset) as u64,
                        pos_ref
                            + (dist
                                * if self.noise {
                                    rand::random::<f64>().clamp(noisiness, 1.0 - noisiness)
                                } else {
                                    0.5
                                }),
                    ),
                );
                offset += 1;
            }
        }
        let pos_ref = self.cells[amount + offset].pos();
        let dist = self.cells[0].pos() - pos_ref;
        if (dist).length_sqr() > self.spatiality {
            self.cells.push_back(Cell::new(
                (amount + offset) as u64,
                pos_ref
                    + (dist
                        * if self.noise {
                            rand::random::<f64>().clamp(noisiness, 1.0 - noisiness)
                        } else {
                            0.5
                        }),
            ))
        }
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas, edges: bool) {
        canvas.set_draw_color(self.color);
        for i in 0..self.cells.len() - 1 {
            canvas
                .draw_line(self.cells[i].pos(), self.cells[i + 1].pos())
                .unwrap();
        }
        canvas
            .draw_line(self.cells[self.cells.len() - 1].pos(), self.cells[0].pos())
            .unwrap();

        canvas.set_draw_color(Color::WHITE);
        if edges {
            for i in 0..self.cells.len() {
                canvas.draw_point(self.cells[i].pos()).unwrap();
            }
        }
    }
}