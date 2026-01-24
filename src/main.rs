use std::array;
use macroquad::prelude::*;
const LARGEUR_BRIQUE: usize = 100;
const HAUTEUR_BRIQUE: usize = 20;
const ESPACEMENT: usize = 5;
const LIGNES: usize = 4;
const COLONNES: usize = 7;

struct Balle {
    pos: (f32, f32),
    vitesse: (f32, f32),
    rayon : f32,
}
struct Tapis {
    pos: f32,
    largeur: f32,
    hauteur: f32,
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
    array::from_fn(|ligne| {
        array::from_fn(|colonne| {
            let x = colonne * (LARGEUR_BRIQUE + ESPACEMENT);
            let y = ligne * (HAUTEUR_BRIQUE + ESPACEMENT) ;
            Brique::from((x as i32, y as i32))
        })
    })
}

#[macroquad::main("Breakout")]
async fn main() {
    let briques = generer_briques();
    let mut tapis = Tapis { pos : 200.0, largeur : 75.0, hauteur : 20.0};
    let mut balle = Balle {
        pos: (200.0, 300.0),
        vitesse: (200.0, -200.0),  // vx, vy
        rayon: 10.0,
    };
    loop {
        clear_background(BLACK);
        //Dessin mur briques
        for ligne in briques.iter() {
            for brique in ligne.iter() {
                if brique.points_de_vie > 0 {
                    draw_rectangle(
                        brique.pos.0 as f32,
                        brique.pos.1 as f32,
                        LARGEUR_BRIQUE as f32,
                        HAUTEUR_BRIQUE as f32,
                        RED,
                    );
                }
            }
        }
        //Maj position tapis
        if is_key_down(KeyCode::Left) {
            tapis.pos -= 5.0;
        }
        if is_key_down(KeyCode::Right) {
            tapis.pos += 5.0;
        }
        //Dessin du tapis
        draw_rectangle(
            tapis.pos as f32,
            screen_height() - tapis.hauteur,  // placer en bas
            tapis.largeur as f32,
            tapis.hauteur,                     // hauteur du tapis
            WHITE,
        );
        //Maj balle
        let dt = get_frame_time();  // get_frame_time() = temps écoulé depuis la dernière frame en secondes
        balle.pos.0 += balle.vitesse.0 * dt;
        balle.pos.1 += balle.vitesse.1 * dt;
        //Rebond gauche-droite
        if balle.pos.0 - balle.rayon < 0.0 //balle à gauche
        || balle.pos.0 + balle.rayon > screen_width(){//balle à droite
            balle.vitesse.0 *= -1.0;
        }
        //Rebond haut
        if balle.pos.1 - balle.rayon < 0.0 {
            balle.vitesse.1 *= -1.0;
        }
        //Rebond bas (sur le tapis)
        if balle.pos.1 + balle.rayon >= screen_height() - tapis.hauteur &&
            balle.pos.0 >= tapis.pos &&
            balle.pos.0 <= (tapis.pos + tapis.largeur){ //ici tapis.pos est le coin haut gauche
                balle.vitesse.1 *= -1.0;
        }
        //Dessin balle
        draw_circle(balle.pos.0, balle.pos.1, balle.rayon, YELLOW);
        //Frame finie, place à la suivante !
        next_frame().await;
    }
}
