use crabe_framework::data::world::World;
use nalgebra::{Point2, Vector2};
use std::error::Error;
use std::process::exit;

/// Projects vector a onto vector b, returning the translation vector to apply
fn project(a: &Vector2<f64>, b: &Vector2<f64>) -> Vector2<f64> {
    (a.dot(b) / b.dot(b)) * b
}

/// non-precise range generator
fn frange(start: f64, end: f64, step: f64) -> Vec<f64> {
    let d = end - start;
    let num_steps = (d * (1. / step)) as i16;
    let mut x = start;
    let mut v = vec![];
    for _ in 0..num_steps {
        v.push(x);
        x += step;
    }

    v
}

// SearchBounds variant of frange
fn frange_sb(sb: &SearchBounds) -> Vec<f64> {
    frange(sb.min, sb.max, sb.step)
}

pub struct SearchBounds {
    pub min: f64,
    pub max: f64,
    pub step: f64
}

mod score {
    use crate::action::opt_pass::project;
    use crabe_framework::data::world::{AllyInfo, EnemyInfo, Robot, RobotMap, World};
    use crabe_math::shape::Circle;
    use nalgebra::{distance, ComplexField, Point2, Vector2};
    use std::collections::HashMap;

    // opponents' perpendicular distance to shoot trajectory.
    // close opponents require less time to block it,
    // so this is a penality to total score.
    fn opponents_block_traj_score(start: &Point2<f64>, p: &Point2<f64>, enemy_positions: &RobotMap<EnemyInfo>) -> f64 {
        // upper bound on max distance check
        // if dist is higher than this const,
        // we consider the enemy will not go block
        // the pass trajectory
        const DIST_MAX_BLOCK_TRAJ_CHECK: f64 = 1.5;

        let traj = p - start;
        enemy_positions.iter()
            .map(|(_, enn_info)| {
                // project enemy position on trajectory
                let vs_enn = enn_info.pose.position - start;
                let translation = project(&vs_enn, &traj);
                let proj = start + translation;

                // opposite directions check
                return if (proj - start).angle(&(p - start)) > 1_f64.to_radians() {
                    // -- ignore enemies whose projected point is outside the trajectory
                    0.
                } else {
                    let d = distance(&proj, &enn_info.pose.position); // TODO: take in account robot radius
                    // if d >= 1.5 { 0. } else {
                        -1. / (1. + d)
                    // } // TODO: tune (for each enemy)
                }
            }).sum()
    }


    /// compute a score using the distance of the closest ally
    /// to where the ball should be shot to
    fn closest_ally_score(shoot_target_loc: &Point2<f64>, allies: &RobotMap<AllyInfo>, ignore_id: u8) -> f64 {
        let ally_positions: Vec<Point2<f64>> = allies.iter()
            .filter(|(id, _)| **id != ignore_id)
            .map(|(_, info)| info.pose.position).collect();
        let min_dist = ally_positions.iter().map(|pos| {
            distance(shoot_target_loc, pos)
        }).fold(f64::INFINITY, f64::min);

        match min_dist.is_infinite() {
            true => 0.0,
            false => if min_dist != 0. {
                if min_dist < 2. { 1. / (1. + min_dist) }
                else { -2. }
            } else {
                0.
            }
        }
    }

    /// Approximate ball cinematic model
    /// With kick power fixed at k (k = ?), gives the speed over time of ball
    ///
    ///TODO: use a real cinematic model based on
    /// - mechanical kicker structure
    /// - accurate distance from kicker to robot
    /// - kicking power value

    fn ball_cin_model(t: f64) -> f64 {
        if t <= 0. { 0. }
        else if t <= 5. { 4. }
        else { -t/2. + 8./2. }
    }

    /// dt_s: current time step in seconds
    /// t_step: delta time step in seconds
    fn ballpos_at_dt(dt__s: f64, traj: Vector2<f64>, t_step__s: f64, b_prev: Point2<f64>) -> Point2<f64> {
        // velocity of ball, direction is linear to trajectory
        let vt = ball_cin_model(dt__s);
        let d = vt * t_step__s;
        b_prev + traj.normalize() * d
    }

    /// Based on opponents current vector speed,
    /// assigns a danger score on whether the opponent will collide
    /// with the pass trajectory
    fn opponents_colliding_with_traj_score(start: &Point2<f64>, shoot_target_loc: &Point2<f64>, world: &World) -> f64 {
        /// The collision score is based on current dt of prediction.
        /// The higher dt is, the more we become unsure of our position prediction of the enemy.
        /// At dt=0s, score is 1.
        /// Score bounds : 0 <= s <= 1.
        /// dt: current time step in seconds
        fn collision_score(dt: usize) -> f64 {
            let dt_s = dt as f64 / 1000.;
            -(-2. * dt_s).exp()
        }

        /// time step delta
        const T_STEP_MS: usize = 100;
        // max prediction, after which we stop predicting collisions
        const MAX_TIME_MS: usize = 1500;

        let enemies = &world.enemies_bot;
        let traj = shoot_target_loc - start;
        let mut enemies_checked: Vec<u8> = vec![];
        let mut collision_scores: Vec<f64> = vec![];

        let mut b_prev = start.clone();

        // estimate possible collision every t_step ms
        for t in (T_STEP_MS..MAX_TIME_MS + T_STEP_MS).step_by(T_STEP_MS) {

            let dt__s = t as f64 / 1000.;
            let t_step__s = T_STEP_MS as f64 / 1000.;
            let b_dt = ballpos_at_dt(dt__s, traj, t_step__s, b_prev);

            // if predicted ball position went farther than its expected location,
            // stop predictions
            if (b_dt - start).norm() > (shoot_target_loc - start).norm() { break; }

            let col_circles_dt =
                collision_circles_at_dt(dt__s,
                                        enemies.iter().
                                            filter(|(id, _)|
                                                !enemies_checked.contains(id)
                                            ).collect::<HashMap<&u8, &Robot<EnemyInfo>>>());

            let cscore = collision_score(t);

            // check collision for each robot
            col_circles_dt.iter().for_each(|(id, circle)| {
                if circle.is_inside(b_dt) {
                    // don't check anymore for this enemy id for next collisions
                    enemies_checked.push(*id);
                    collision_scores.push(cscore);
                }
            });

            // stop early if collisions were found with every robot
            if enemies_checked.len() == enemies.len() { break; }

            b_prev = b_dt;
        }

        collision_scores.iter().sum()
    }

    /// dt: seconds in f64
    /// enemies: robots to check collision with
    fn collision_circles_at_dt(dt: f64, enemies: HashMap<&u8, &Robot<EnemyInfo>>) -> Vec<(u8, Circle)> {
        // how confident we are in predicting the current circle
        // if this is one, point gets offset of velocity `--v-> * dt`,
        // meaning we're 100% sure that after `dt` time, the enemy robot will be at
        // the predicted location. As robots can sometimes change their trajectory,
        // this factor is used to reduce the progression of the predicted positions.
        // Previous result would assume that the considered robot is able to directly stop
        // in a very short time, to go completely another way.
        const CONFIDENCE: f64 = 0.5;

        // Supposes that enemy robots have an average speed.
        // When a robot is still, this will be the only factor in the
        // predicted collision circle calculus
        const AVG_SPEED: f64 = 1.5; // m/s

        enemies.iter()
            .map(|(_, info)|
                (info.id, info.velocity.linear, info.pose.position)
            )
            .map(|(id, vel, p)| {
                let d = AVG_SPEED * dt;  // TODO: distance travaled after a long time might be too high
                let p_predicted = p + vel * dt * CONFIDENCE;
                (id, Circle { center: p_predicted, radius: d })
            }).collect()
    }

    pub(super) fn compute_score(
        world: &World,
        from: &Point2<f64>,
        shoot_target_loc: &Point2<f64>,
        ignore_ally_id: u8
    ) -> f64 {
        [
            1. * opponents_block_traj_score(from, shoot_target_loc, &world.enemies_bot),
            1. * closest_ally_score(shoot_target_loc, &world.allies_bot, ignore_ally_id),
            1. * opponents_colliding_with_traj_score(from, shoot_target_loc, &world),
            1. * progress_on_field_score(shoot_target_loc)
        ].iter().sum()
    }

    fn progress_on_field_score(shoot_target_loc: &Point2<f64>) -> f64 {
        //f(x) = -x(x-9/2)  if x > 0.
        //f(x) = -x(x+9/2)  otherwise
        let x = shoot_target_loc.x;
        if x > 0. {
            (-x * (x - 4.5)) / 5.
        } else {
            (-x * (x + 4.5)) / 5.
        }
    }
}


pub struct Node {
    pub p: Point2<f64>,
    pub score: f64
}

pub struct OptimalPassMoveTo {
    pub graph: Vec<Node>,
}

impl OptimalPassMoveTo {
    pub fn new(world: &World, from: Point2<f64>, x_bounds: SearchBounds, y_bounds: SearchBounds) -> Self {
        
        Self {
            graph: gen_graph(world, from, x_bounds, y_bounds),
        }
    }
}

fn gen_graph(world: &World, from: Point2<f64>, x_bounds: SearchBounds, y_bounds: SearchBounds) -> Vec<Node> {
    let mut g: Vec<Node> = vec![];
    
    if world.allies_bot.len() == 0 { return g; }
    
    // let p = Point2::new(2.0, 0.);
    // g.push(Node { p, score: score::compute_score(world, &from, &p) });

    let mut i_x = 0;
    let mut i_y = 0;
    for x in frange_sb(&x_bounds) {
        i_y = 0;
        i_x += 1;
        for y in frange_sb(&y_bounds) {
            i_y += 1;
            let p = Point2::new(x, y);
            let score = score::compute_score(world, &from, &p, 0);
            g.push(Node { p, score });
        }
    }

    write_csv(&g, x_bounds, y_bounds, i_x, i_y).expect("Failed to generate graph");
    write_world(world).expect("Failed to generate graph");
    exit(0);

    g
}

fn write_world(world: &World) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("world.csv")?;
    wtr.write_record(&["ally_x", "ally_y", "enemy_x", "enemy_y"])?;
    for i in 0..world.allies_bot.len() {
        let ally = world.allies_bot.get(&(i as u8)).unwrap();
        let enemy = world.enemies_bot.get(&(i as u8)).unwrap();
        wtr.write_record(&[
            ally.pose.position.x.to_string(),
            ally.pose.position.y.to_string(),
            enemy.pose.position.x.to_string(),
            enemy.pose.position.y.to_string(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

fn write_csv(graph: &Vec<Node>, x_bounds: SearchBounds, y_bounds: SearchBounds, i_x: i32, i_y: i32) -> Result<(), Box<dyn Error>> {
    let mut bounds_wtr = csv::Writer::from_path("bounds.csv")?;
    bounds_wtr.write_record(&["x_step", "y_step", "i_x", "i_y"])?;
    bounds_wtr.write_record(&[x_bounds.step.to_string(), y_bounds.step.to_string(), i_x.to_string(), i_y.to_string()])?;
    bounds_wtr.flush()?;

    let mut wtr = csv::Writer::from_path("data.csv")?;
    wtr.write_record(&["x", "y", "score"])?;
    graph.iter().for_each(|node| {
        wtr.write_record(&[
            node.p.x.to_string(),
            node.p.y.to_string(),
            node.score.to_string(),
        ]).expect("[CSV Write] Neko doesn't want to write");
    });
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_vec_on_vec() {
        let base = Vector2::new(0., 2.);
        let v = Vector2::new(1., 1.);
        
        assert_eq!(project(&v, &base), Vector2::new(0., 1.));
        assert_eq!(project(&base, &v), Vector2::new(1., 1.));

        let a = Point2::new(6., 4.);
        let b = Point2::new(7., 5.);
        let c = Point2::new(8., 4.);
        let translation = project(&(b - a), &(c - a));
        assert_eq!(translation, Vector2::new(1., 0.));
        let proj = a + translation;
        assert_eq!(proj, Point2::new(7., 4.));
    }

}