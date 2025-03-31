use r2r::{
    Node, Publisher, QosProfile,
    geometry_msgs::msg::{Twist, Vector3},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};

type NavNode = Arc<Mutex<Node>>;

pub async fn get_pub(node: NavNode) -> Publisher<Twist> {
    let mut lock = node.lock().await;

    let publisher = lock
        .create_publisher::<Twist>("/cmd_vel", QosProfile::default())
        .unwrap();

    publisher
}

// move x units in x direction and y units in y direction
pub async fn nav_move(node: NavNode, distance_x: f64, distance_y: f64) {
    let publisher = get_pub(node).await;

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
            x: 0.0,
            y: 0.0,
            z: a_x,
        }, // Rotate slightly
    };

    // Publish the initial move message
    match publisher.publish(&twist) {
        Ok(_) => println!(
            "Published: linear = {}, angular = {}",
            twist.linear.x, twist.angular.z
        ),
        Err(e) => eprintln!("Failed to publish intial move instructions: {}", e),
    }

    // Sleep for time needed to reach distance
    sleep(Duration::from_secs(travel_time)).await;
}

pub async fn rotate360(node: NavNode) {
    let cl = Arc::clone(&node);
    let publisher = get_pub(cl).await;

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

    nav_stop(node);
}

pub async fn nav_stop(node: NavNode) {
    let publisher = get_pub(node).await;

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
