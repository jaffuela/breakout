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
    let mut briques = generer_briques();
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
        let ancienne_pos = balle.pos;
        //Maj balle
        let dt = get_frame_time();  // get_frame_time() = temps écoulé depuis la dernière frame en secondes
        balle.pos.0 += balle.vitesse.0 * dt;
        balle.pos.1 += balle.vitesse.1 * dt;

        //Rebonds briques haut-bas (gestion continu)
        for ligne in briques.iter_mut() {
            for brique in ligne.iter_mut() {
                if brique.points_de_vie > 0 {
                    let bx = brique.pos.0 as f32;
                    let by = brique.pos.1 as f32;
                    let bw = LARGEUR_BRIQUE as f32;
                    let bh = HAUTEUR_BRIQUE as f32;

                    //GESTION COLLISIONS VERTICALES

                    let balle_x = balle.pos.0;
                    let dans_x = balle_x >= bx && balle_x <= bx + bw;
                    if dans_x{
                        if ancienne_pos.1 + balle.rayon < by + bh &&
                            balle.pos.1 + balle.rayon >= by + bh {
                            //la balle traverse la brique par le dessous
                            balle.vitesse.1 *= -1.0;
                            brique.points_de_vie -=1 ;
                        }
                        if ancienne_pos.1 - balle.rayon > by &&
                            balle.pos.1 - balle.rayon <= by  {
                            //La balle traverse la brique par le haut
                            balle.vitesse.1 *= -1.0;
                            brique.points_de_vie -= 1;
                        }
                    }

                    //GESTION COLLISIONS HORIZONTALES
                    let balle_y = balle.pos.1;
                    if balle_y + balle.rayon >= by && balle_y - balle.rayon <= by + bh {
                        // collision côté gauche
                        if ancienne_pos.0 - balle.rayon > bx &&
                            balle_x - balle.rayon <= bx
                        {
                            balle.vitesse.0 *= -1.0;
                            balle.pos.0 = bx - balle.rayon; // repositionnement
                            brique.points_de_vie -= 1;
                        }

                        // collision côté droit
                        if ancienne_pos.0 + balle.rayon < bx + bw &&
                            balle_x + balle.rayon >= bx + bw
                        {
                            balle.vitesse.0 *= -1.0;
                            balle.pos.0 = bx + bw + balle.rayon; // repositionnement
                            brique.points_de_vie -= 1;
                        }
                    }
                }
            }
        }

        //Rebond gauche mur
        if balle.pos.0 - balle.rayon < 0.0 //balle à gauche
        {
            balle.vitesse.0 *= -1.0;
            balle.pos.0 = balle.rayon; //repositionnement
        }
        //Rebond droite
        if balle.pos.0 + balle.rayon > screen_width(){
            balle.vitesse.0 *= -1.0;
            balle.pos.0 = screen_width() - balle.rayon; //repositionnement
        }

        //Rebond mur haut
        if balle.pos.1 - balle.rayon < 0.0 {
            balle.vitesse.1 *= -1.0;
            balle.pos.1 = balle.rayon; //repositionnement
        }
        //Rebond bas (sur le tapis, gestion continu)
        if balle.vitesse.1 > 0.0 {
            let y_tapis = screen_height() - tapis.hauteur;
            //Segment coupe la ligne y_tapis ?
            if ancienne_pos.1 + balle.rayon < y_tapis //la balle est au dessus du tapis
                && balle.pos.1 + balle.rayon >= y_tapis//la balle finit en dessous du tapis
                && balle.pos.0 >= tapis.pos && balle.pos.0 <= tapis.pos + tapis.largeur
                //Le centre de la balle est-il dans les limites du tapis (horizontalement)
                {
                    balle.vitesse.1 *= -1.0;
                    balle.pos.1 = y_tapis - balle.rayon; //repositionnement
            }
        }
        //Dessin balle
        draw_circle(balle.pos.0, balle.pos.1, balle.rayon, YELLOW);
        //Frame finie, place à la suivante !
        next_frame().await;
    }
}
