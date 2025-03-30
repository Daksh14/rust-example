use r2r::{
    Publisher,
    geometry_msgs::msg::{Twist, Vector3},
};
use tokio::time::{Duration, sleep};

// move x units in x direction and y units in y direction
pub async fn nav_move(publisher: &Publisher<Twist>, distance_x: f64, distance_y: f64) {
    let speed = 0.2;

    let angle = distance_y.atan2(distance_x);

    let v_x = speed * angle.cos();
    let a_x = speed * angle.sin();

    // Time to move in a straight line
    let distance = (v_x.powi(2) + a_x.powi(2)).sqrt();
    let travel_time = ((distance / speed) as u64) * 2;

    println!("Travel time: {}", travel_time);
    println!("distance: {}", distance);
    println!("v_x: {}", v_x);
    println!("a_x: {}", a_x);

    let twist = Twist {
        linear: Vector3 {
            x: v_x,
            y: 0.0,
            z: 0.0,
        }, // Move forward
        angular: Vector3 {
            x: a_x,
            y: 0.0,
            z: 0.3,
        }, // Rotate slightly
    };

    // Publish the initial move message
    match publisher.publish(&twist) {
        Ok(_) => println!(
            "Published: linear = {}, angular = {}",
            twist.linear.x, twist.angular.x
        ),
        Err(e) => eprintln!("Failed to publish intial move instructions: {}", e),
    }

    // Sleep for time needed to reach distance
    sleep(Duration::from_secs(travel_time)).await;

    nav_stop(publisher);
}

pub async fn rotate360(publisher: &Publisher<Twist>) {
    let twist = Twist {
        linear: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }, // Move forward
        angular: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.3,
        }, // Rotate slightly
    };

    // Publish the rotation message
    match publisher.publish(&twist) {
        Ok(_) => println!("Rotating instruction sent"),
        Err(e) => eprintln!("Failed to publish 360 rotating instructions {}", e),
    }

    // Sleep for time needed to reach distance
    sleep(Duration::from_secs(5)).await;

    nav_stop(publisher);
}

pub fn nav_stop(publisher: &Publisher<Twist>) {
    let twist = Twist {
        linear: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        angular: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };

    // Publish the rotation message
    match publisher.publish(&twist) {
        Ok(_) => println!("Stopping instruction sent"),
        Err(e) => eprintln!("Failed to stop the bot, this is bad {}", e),
    };
}
