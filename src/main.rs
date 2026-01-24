use std::iter;
use std::array;
use macroquad::prelude::*;
const LARGEUR_BRIQUE: usize = 50;
const HAUTEUR_BRIQUE: usize = 20;
const ESPACEMENT: usize = 5;
const LIGNES: usize = 4;
const COLONNES: usize = 8;

struct Balle {
    pos: (f32, f32),
    vitesse: (f32, f32),
}
struct Tapis {
    pos: f32,
    largeur: f32,
}

struct Brique {
    pos: (i32, i32),
    points_de_vie: u32,
}
impl From<(i32, i32)> for Brique {
    fn from(pos: (i32, i32)) -> Self {
        Brique { pos, points_de_vie: 1 }
    }
}

fn generer_briques() -> [[Brique;COLONNES];LIGNES] {
    array::from_fn(move |ligne| {
        array::from_fn(move |colonne| {
            let x = colonne * (LARGEUR_BRIQUE + ESPACEMENT);
            let y = ligne * (HAUTEUR_BRIQUE + ESPACEMENT) ;
            Brique::from((x as i32, y as i32))
        })
    })
}

fn main() {
    println!("Hello, world!");
    let balle = Balle { pos: (200.0, 300.0), vitesse: (150.0, -150.0) };
    let paddle = Tapis { pos: 200.0, largeur: 80.0 };
    let brique = Brique::from((50, 100));
    let briques = generer_briques();

    for ligne in briques.iter() {
        for b in ligne.iter() {
            println!("{:?}", b.pos);
        }
    }

}