use std::collections::VecDeque;
use std::f32::consts::PI;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::cells::Cell;
use crate::vectors::Vector2;

pub struct Organism {
    cells: VecDeque<Cell>,
    color: Color,
    spatiality: f64,
}

impl Organism {
    pub fn new(pos: Vector2, size: u32, radius: f64, spatiality: f64) -> Self {
        let mut cells = VecDeque::new();
        let angle = (2.0 * PI as f64 / size as f64);
        for i in 0..size {
            cells.push_back(Cell::new(
                i as u64,
                pos + Vector2::new(
                    (angle * (i as f64)).cos() * radius,
                    (angle * (i as f64)).sin() * radius,
                ),
            ))
        }

        Self {
            cells,
            color: Color::RGB(255, 0, 0),
            spatiality,
        }
    }

    pub fn expand(&mut self) {
        for i in 0..self.cells.len() {
            let mut force = Vector2::default();
            for c in self.cells.iter() {
                if c.id != self.cells[i].id {
                    let dist = self.cells[i].position - c.position;
                    force += dist.mag(1f64 / dist.length_sqr());
                }
            }

            force = force * 10f64;

            let accel = force * (1.0 / self.cells[i].mass);
            self.cells[i].velocity += accel;
            self.cells[i].velocity = self.cells[i].velocity * 0.5;
        }

        for i in 0..self.cells.len() {
            self.cells[i].react();
        }
    }
    pub fn fill_gaps(&mut self) {
        let mut offset = 0;
        let amount = self.cells.len() - 1;
        for i in 0..amount {
            let pos_ref = self.cells[i + offset].pos();
            let dist = self.cells[i + offset + 1].pos() - pos_ref;
            if dist.length_sqr() > self.spatiality {
                self.cells.insert(
                    i + offset + 1,
                    Cell::new((amount + 1 + offset) as u64, pos_ref + (dist * 0.5)),
                );
                offset += 1;
            }
        }
        let pos_ref = self.cells[amount + offset].pos();
        let dist = self.cells[0].pos() - pos_ref;
        if (dist).length_sqr() > self.spatiality {
            self.cells
                .push_back(Cell::new((amount + offset) as u64, pos_ref + (dist * 0.5)))
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

        canvas.set_draw_color(Color::BLUE);
        if edges {
            for i in 0..self.cells.len() {
                canvas.draw_point(self.cells[i].pos()).unwrap();
            }
        }
    }
}