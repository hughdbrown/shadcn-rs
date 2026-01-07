//! Touch gesture utilities for mobile interactions
//!
//! Provides utilities for detecting and handling touch gestures like swipes.

use web_sys::{Touch, TouchEvent};

/// Direction of a swipe gesture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwipeDirection {
    /// Swipe left
    Left,
    /// Swipe right
    Right,
    /// Swipe up
    Up,
    /// Swipe down
    Down,
}

/// Configuration for swipe detection
#[derive(Debug, Clone, Copy)]
pub struct SwipeConfig {
    /// Minimum distance in pixels to register as a swipe
    pub min_distance: f64,

    /// Maximum time in milliseconds for a swipe
    pub max_duration: f64,

    /// Velocity threshold (pixels per millisecond)
    pub min_velocity: f64,
}

impl Default for SwipeConfig {
    fn default() -> Self {
        Self {
            min_distance: 30.0,  // 30px minimum
            max_duration: 300.0, // 300ms maximum
            min_velocity: 0.1,   // 0.1 px/ms minimum
        }
    }
}

/// Touch point data
#[derive(Debug, Clone, Copy)]
pub struct TouchPoint {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Timestamp
    pub time: f64,
}

impl TouchPoint {
    /// Create a TouchPoint from a Touch object
    pub fn from_touch(touch: &Touch, time: f64) -> Self {
        Self {
            x: touch.client_x() as f64,
            y: touch.client_y() as f64,
            time,
        }
    }

    /// Calculate distance to another point
    pub fn distance_to(&self, other: &TouchPoint) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate time difference to another point
    pub fn duration_to(&self, other: &TouchPoint) -> f64 {
        (other.time - self.time).abs()
    }

    /// Calculate velocity to another point
    pub fn velocity_to(&self, other: &TouchPoint) -> f64 {
        let distance = self.distance_to(other);
        let duration = self.duration_to(other);
        if duration > 0.0 {
            distance / duration
        } else {
            0.0
        }
    }
}

/// Detect swipe direction from two touch points
///
/// # Examples
///
/// ```
/// use shadcn_rs::utils::{TouchPoint, detect_swipe, SwipeConfig};
///
/// let start = TouchPoint { x: 100.0, y: 100.0, time: 0.0 };
/// let end = TouchPoint { x: 200.0, y: 105.0, time: 100.0 };
/// let config = SwipeConfig::default();
///
/// if let Some(direction) = detect_swipe(&start, &end, &config) {
///     println!("Swiped: {:?}", direction);
/// }
/// ```
pub fn detect_swipe(
    start: &TouchPoint,
    end: &TouchPoint,
    config: &SwipeConfig,
) -> Option<SwipeDirection> {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let distance = start.distance_to(end);
    let duration = start.duration_to(end);
    let velocity = start.velocity_to(end);

    // Check if gesture meets requirements
    if distance < config.min_distance {
        return None;
    }

    if duration > config.max_duration {
        return None;
    }

    if velocity < config.min_velocity {
        return None;
    }

    // Determine direction based on dominant axis
    let abs_dx = dx.abs();
    let abs_dy = dy.abs();

    if abs_dx > abs_dy {
        // Horizontal swipe
        if dx > 0.0 {
            Some(SwipeDirection::Right)
        } else {
            Some(SwipeDirection::Left)
        }
    } else {
        // Vertical swipe
        if dy > 0.0 {
            Some(SwipeDirection::Down)
        } else {
            Some(SwipeDirection::Up)
        }
    }
}

/// Extract the first touch from a TouchEvent
pub fn get_first_touch(event: &TouchEvent) -> Option<Touch> {
    event.touches().get(0)
}

/// Get current timestamp in milliseconds
pub fn get_timestamp() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

/// Create a TouchPoint from a TouchEvent
pub fn touch_point_from_event(event: &TouchEvent) -> Option<TouchPoint> {
    let touch = get_first_touch(event)?;
    let time = get_timestamp();
    Some(TouchPoint::from_touch(&touch, time))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swipe_config_default() {
        let config = SwipeConfig::default();
        assert_eq!(config.min_distance, 30.0);
        assert_eq!(config.max_duration, 300.0);
        assert_eq!(config.min_velocity, 0.1);
    }

    #[test]
    fn test_touch_point_distance() {
        let p1 = TouchPoint {
            x: 0.0,
            y: 0.0,
            time: 0.0,
        };
        let p2 = TouchPoint {
            x: 3.0,
            y: 4.0,
            time: 100.0,
        };

        let distance = p1.distance_to(&p2);
        assert!((distance - 5.0).abs() < 0.001); // 3-4-5 triangle
    }

    #[test]
    fn test_touch_point_duration() {
        let p1 = TouchPoint {
            x: 0.0,
            y: 0.0,
            time: 100.0,
        };
        let p2 = TouchPoint {
            x: 0.0,
            y: 0.0,
            time: 250.0,
        };

        assert_eq!(p1.duration_to(&p2), 150.0);
    }

    #[test]
    fn test_touch_point_velocity() {
        let p1 = TouchPoint {
            x: 0.0,
            y: 0.0,
            time: 0.0,
        };
        let p2 = TouchPoint {
            x: 100.0,
            y: 0.0,
            time: 100.0,
        };

        let velocity = p1.velocity_to(&p2);
        assert_eq!(velocity, 1.0); // 100px / 100ms = 1.0 px/ms
    }

    #[test]
    fn test_detect_swipe_right() {
        let start = TouchPoint {
            x: 0.0,
            y: 0.0,
            time: 0.0,
        };
        let end = TouchPoint {
            x: 100.0,
            y: 5.0,
            time: 100.0,
        };
        let config = SwipeConfig::default();

        let direction = detect_swipe(&start, &end, &config);
        assert_eq!(direction, Some(SwipeDirection::Right));
    }

    #[test]
    fn test_detect_swipe_left() {
        let start = TouchPoint {
            x: 100.0,
            y: 0.0,
            time: 0.0,
        };
        let end = TouchPoint {
            x: 0.0,
            y: 5.0,
            time: 100.0,
        };
        let config = SwipeConfig::default();

        let direction = detect_swipe(&start, &end, &config);
        assert_eq!(direction, Some(SwipeDirection::Left));
    }

    #[test]
    fn test_detect_swipe_up() {
        let start = TouchPoint {
            x: 0.0,
            y: 100.0,
            time: 0.0,
        };
        let end = TouchPoint {
            x: 5.0,
            y: 0.0,
            time: 100.0,
        };
        let config = SwipeConfig::default();

        let direction = detect_swipe(&start, &end, &config);
        assert_eq!(direction, Some(SwipeDirection::Up));
    }

    #[test]
    fn test_detect_swipe_down() {
        let start = TouchPoint {
            x: 0.0,
            y: 0.0,
            time: 0.0,
        };
        let end = TouchPoint {
            x: 5.0,
            y: 100.0,
            time: 100.0,
        };
        let config = SwipeConfig::default();

        let direction = detect_swipe(&start, &end, &config);
        assert_eq!(direction, Some(SwipeDirection::Down));
    }

    #[test]
    fn test_detect_swipe_too_short() {
        let start = TouchPoint {
            x: 0.0,
            y: 0.0,
            time: 0.0,
        };
        let end = TouchPoint {
            x: 10.0,
            y: 0.0,
            time: 100.0,
        };
        let config = SwipeConfig::default();

        let direction = detect_swipe(&start, &end, &config);
        assert_eq!(direction, None); // Distance < min_distance
    }

    #[test]
    fn test_detect_swipe_too_slow() {
        let start = TouchPoint {
            x: 0.0,
            y: 0.0,
            time: 0.0,
        };
        let end = TouchPoint {
            x: 100.0,
            y: 0.0,
            time: 500.0,
        };
        let config = SwipeConfig::default();

        let direction = detect_swipe(&start, &end, &config);
        assert_eq!(direction, None); // Duration > max_duration
    }
}
