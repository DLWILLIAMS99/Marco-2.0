//! Touch Input System for Marco 2.0
//!
//! Provides comprehensive touch input handling including multi-touch gestures,
//! touch-optimized tools, and gesture recognition for mobile and tablet interfaces.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use glam::Vec2;
use std::time::{Duration, Instant};

/// Unique identifier for touch points
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TouchId(pub u64);

/// Touch point state and properties
#[derive(Debug, Clone)]
pub struct TouchState {
    pub id: TouchId,
    pub position: Vec2,
    pub start_position: Vec2,
    pub start_time: Instant,
    pub pressure: f32,
    pub radius: f32,
    pub is_active: bool,
}

impl TouchState {
    pub fn new(id: TouchId, position: Vec2) -> Self {
        Self {
            id,
            position,
            start_position: position,
            start_time: Instant::now(),
            pressure: 1.0,
            radius: 10.0,
            is_active: true,
        }
    }
    
    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    pub fn distance_from_start(&self) -> f32 {
        self.position.distance(self.start_position)
    }
    
    pub fn velocity(&self) -> Vec2 {
        let duration = self.duration().as_secs_f32();
        if duration > 0.0 {
            (self.position - self.start_position) / duration
        } else {
            Vec2::ZERO
        }
    }
}

/// Recognized touch gestures
#[derive(Debug, Clone, PartialEq)]
pub enum TouchGesture {
    Tap {
        position: Vec2,
        touch_count: usize,
    },
    DoubleTap {
        position: Vec2,
    },
    LongPress {
        position: Vec2,
        duration: Duration,
    },
    Pan {
        start_position: Vec2,
        current_position: Vec2,
        delta: Vec2,
    },
    Pinch {
        center: Vec2,
        scale: f32,
        rotation: f32,
    },
    Swipe {
        start_position: Vec2,
        end_position: Vec2,
        direction: SwipeDirection,
        velocity: f32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl SwipeDirection {
    pub fn from_vector(delta: Vec2) -> Option<Self> {
        let abs_x = delta.x.abs();
        let abs_y = delta.y.abs();
        
        if abs_x > abs_y {
            if delta.x > 0.0 {
                Some(SwipeDirection::Right)
            } else {
                Some(SwipeDirection::Left)
            }
        } else if abs_y > 20.0 { // Minimum swipe distance
            if delta.y > 0.0 {
                Some(SwipeDirection::Down)
            } else {
                Some(SwipeDirection::Up)
            }
        } else {
            None
        }
    }
}

/// Touch gesture recognition thresholds
#[derive(Debug, Clone)]
pub struct GestureThresholds {
    pub tap_max_distance: f32,
    pub tap_max_duration: Duration,
    pub double_tap_max_interval: Duration,
    pub long_press_duration: Duration,
    pub pan_min_distance: f32,
    pub swipe_min_velocity: f32,
    pub pinch_min_distance: f32,
}

impl Default for GestureThresholds {
    fn default() -> Self {
        Self {
            tap_max_distance: 10.0,
            tap_max_duration: Duration::from_millis(500),
            double_tap_max_interval: Duration::from_millis(300),
            long_press_duration: Duration::from_millis(800),
            pan_min_distance: 5.0,
            swipe_min_velocity: 200.0, // pixels per second
            pinch_min_distance: 20.0,
        }
    }
}

/// Touch-optimized tool set for mobile interfaces
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TouchTool {
    Select,
    Pan,
    Zoom,
    Draw,
    Erase,
    Text,
    Shape,
    Gesture,
}

impl TouchTool {
    pub fn icon(&self) -> &'static str {
        match self {
            TouchTool::Select => "👆",
            TouchTool::Pan => "✋",
            TouchTool::Zoom => "🔍",
            TouchTool::Draw => "✏️",
            TouchTool::Erase => "🧽",
            TouchTool::Text => "📝",
            TouchTool::Shape => "📐",
            TouchTool::Gesture => "👋",
        }
    }
    
    pub fn supports_multi_touch(&self) -> bool {
        matches!(self, TouchTool::Pan | TouchTool::Zoom | TouchTool::Gesture)
    }
}

/// Touch tool configuration
#[derive(Debug, Clone)]
pub struct TouchToolSet {
    pub active_tool: TouchTool,
    pub tool_size: f32,
    pub touch_radius: f32,
    pub pressure_sensitivity: bool,
}

impl Default for TouchToolSet {
    fn default() -> Self {
        Self {
            active_tool: TouchTool::Select,
            tool_size: 48.0, // Large enough for touch
            touch_radius: 24.0,
            pressure_sensitivity: false,
        }
    }
}

/// Gesture recognition state machine
#[derive(Debug, Clone)]
enum GestureState {
    Idle,
    TouchDown { touch_id: TouchId, start_time: Instant },
    PotentialTap { touch_id: TouchId, start_time: Instant },
    PotentialDoubleTap { first_tap_time: Instant, first_tap_pos: Vec2 },
    LongPressWaiting { touch_id: TouchId, start_time: Instant },
    Panning { touch_id: TouchId, last_position: Vec2 },
    Pinching { touch1: TouchId, touch2: TouchId, initial_distance: f32, initial_center: Vec2 },
    Swiping { touch_id: TouchId, start_position: Vec2 },
}

/// Main touch input handler
#[derive(Debug)]
pub struct TouchHandler {
    /// Active touch points
    active_touches: HashMap<TouchId, TouchState>,
    
    /// Gesture recognition state
    gesture_state: GestureState,
    
    /// Gesture recognition thresholds
    thresholds: GestureThresholds,
    
    /// Touch-optimized tool set
    tools: TouchToolSet,
    
    /// Recognized gestures (events to be processed)
    pending_gestures: Vec<TouchGesture>,
    
    /// Last gesture recognition time for filtering
    last_gesture_time: Instant,
}

impl Default for TouchHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl TouchHandler {
    pub fn new() -> Self {
        Self {
            active_touches: HashMap::new(),
            gesture_state: GestureState::Idle,
            thresholds: GestureThresholds::default(),
            tools: TouchToolSet::default(),
            pending_gestures: Vec::new(),
            last_gesture_time: Instant::now(),
        }
    }
    
    /// Handle touch down event
    pub fn touch_down(&mut self, id: TouchId, position: Vec2, pressure: f32) {
        let touch = TouchState {
            id,
            position,
            start_position: position,
            start_time: Instant::now(),
            pressure,
            radius: self.tools.touch_radius,
            is_active: true,
        };
        
        self.active_touches.insert(id, touch);
        self.update_gesture_state();
        
        tracing::debug!("Touch down: {:?} at {:?}", id, position);
    }
    
    /// Handle touch move event
    pub fn touch_move(&mut self, id: TouchId, position: Vec2, pressure: f32) {
        if let Some(touch) = self.active_touches.get_mut(&id) {
            touch.position = position;
            touch.pressure = pressure;
            self.update_gesture_state();
        }
    }
    
    /// Handle touch up event
    pub fn touch_up(&mut self, id: TouchId) {
        if let Some(touch) = self.active_touches.remove(&id) {
            self.handle_touch_end(&touch);
            self.update_gesture_state();
        }
        
        tracing::debug!("Touch up: {:?}", id);
    }
    
    /// Handle touch cancellation
    pub fn touch_cancel(&mut self, id: TouchId) {
        self.active_touches.remove(&id);
        self.gesture_state = GestureState::Idle;
        tracing::debug!("Touch cancelled: {:?}", id);
    }
    
    /// Update gesture recognition state machine
    fn update_gesture_state(&mut self) {
        let active_count = self.active_touches.len();
        let now = Instant::now();
        
        // Clone the current state to avoid borrow checker issues
        let current_state = self.gesture_state.clone();
        
        match (current_state, active_count) {
            (GestureState::Idle, 1) => {
                if let Some((&id, touch)) = self.active_touches.iter().next() {
                    self.gesture_state = GestureState::TouchDown { 
                        touch_id: id, 
                        start_time: touch.start_time 
                    };
                }
            },
            
            (GestureState::TouchDown { touch_id, start_time }, 1) => {
                if let Some(touch) = self.active_touches.get(&touch_id) {
                    let duration = now.duration_since(start_time);
                    let distance = touch.distance_from_start();
                    
                    if duration >= self.thresholds.long_press_duration {
                        self.recognize_long_press(touch.position, duration);
                        self.gesture_state = GestureState::Idle;
                    } else if distance >= self.thresholds.pan_min_distance {
                        if self.tools.active_tool.supports_multi_touch() {
                            self.gesture_state = GestureState::Panning { 
                                touch_id, 
                                last_position: touch.position 
                            };
                        }
                    }
                }
            },
            
            (GestureState::Panning { touch_id, last_position }, 1) => {
                if let Some(touch) = self.active_touches.get(&touch_id) {
                    let start_pos = touch.start_position;
                    let current_pos = touch.position;
                    let delta = current_pos - last_position;
                    self.recognize_pan(start_pos, current_pos, delta);
                    self.gesture_state = GestureState::Panning { 
                        touch_id, 
                        last_position: current_pos 
                    };
                }
            },
            
            (_, 2) => {
                if self.tools.active_tool.supports_multi_touch() {
                    self.start_pinch_gesture();
                }
            },
            
            (GestureState::Pinching { touch1, touch2, initial_distance, initial_center }, 2) => {
                if let (Some(t1), Some(t2)) = (self.active_touches.get(&touch1), self.active_touches.get(&touch2)) {
                    let current_distance = t1.position.distance(t2.position);
                    let current_center = (t1.position + t2.position) * 0.5;
                    let scale = current_distance / initial_distance;
                    
                    self.recognize_pinch(current_center, scale, 0.0); // TODO: Calculate rotation
                }
            },
            
            _ => {
                if active_count == 0 {
                    self.gesture_state = GestureState::Idle;
                }
            }
        }
    }
    
    /// Handle end of touch for gesture recognition
    fn handle_touch_end(&mut self, touch: &TouchState) {
        let duration = touch.duration();
        let distance = touch.distance_from_start();
        let velocity = touch.velocity().length();
        
        match &self.gesture_state {
            GestureState::TouchDown { .. } | GestureState::PotentialTap { .. } => {
                if duration <= self.thresholds.tap_max_duration && distance <= self.thresholds.tap_max_distance {
                    self.handle_potential_tap(touch.position);
                }
            },
            
            GestureState::Panning { .. } => {
                if velocity >= self.thresholds.swipe_min_velocity {
                    if let Some(direction) = SwipeDirection::from_vector(touch.position - touch.start_position) {
                        self.recognize_swipe(touch.start_position, touch.position, direction, velocity);
                    }
                }
            },
            
            _ => {}
        }
    }
    
    /// Handle potential tap for double-tap detection
    fn handle_potential_tap(&mut self, position: Vec2) {
        let now = Instant::now();
        
        match &self.gesture_state {
            GestureState::PotentialDoubleTap { first_tap_time, first_tap_pos } => {
                let interval = now.duration_since(*first_tap_time);
                let distance = position.distance(*first_tap_pos);
                
                if interval <= self.thresholds.double_tap_max_interval && distance <= self.thresholds.tap_max_distance {
                    self.recognize_double_tap(position);
                } else {
                    self.recognize_tap(position, 1);
                }
                self.gesture_state = GestureState::Idle;
            },
            
            _ => {
                self.gesture_state = GestureState::PotentialDoubleTap { 
                    first_tap_time: now, 
                    first_tap_pos: position 
                };
                
                // Set a timer to recognize single tap if no second tap comes
                // This would typically be handled by a timer system
            }
        }
    }
    
    /// Start pinch gesture recognition
    fn start_pinch_gesture(&mut self) {
        let touches: Vec<_> = self.active_touches.iter().take(2).collect();
        if touches.len() == 2 {
            let (id1, touch1) = touches[0];
            let (id2, touch2) = touches[1];
            
            let initial_distance = touch1.position.distance(touch2.position);
            let initial_center = (touch1.position + touch2.position) * 0.5;
            
            if initial_distance >= self.thresholds.pinch_min_distance {
                self.gesture_state = GestureState::Pinching {
                    touch1: *id1,
                    touch2: *id2,
                    initial_distance,
                    initial_center,
                };
            }
        }
    }
    
    /// Recognize tap gesture
    fn recognize_tap(&mut self, position: Vec2, touch_count: usize) {
        self.pending_gestures.push(TouchGesture::Tap { position, touch_count });
        tracing::debug!("Recognized tap at {:?}", position);
    }
    
    /// Recognize double tap gesture
    fn recognize_double_tap(&mut self, position: Vec2) {
        self.pending_gestures.push(TouchGesture::DoubleTap { position });
        tracing::debug!("Recognized double tap at {:?}", position);
    }
    
    /// Recognize long press gesture
    fn recognize_long_press(&mut self, position: Vec2, duration: Duration) {
        self.pending_gestures.push(TouchGesture::LongPress { position, duration });
        tracing::debug!("Recognized long press at {:?}", position);
    }
    
    /// Recognize pan gesture
    fn recognize_pan(&mut self, start_position: Vec2, current_position: Vec2, delta: Vec2) {
        self.pending_gestures.push(TouchGesture::Pan { 
            start_position, 
            current_position, 
            delta 
        });
    }
    
    /// Recognize pinch gesture
    fn recognize_pinch(&mut self, center: Vec2, scale: f32, rotation: f32) {
        self.pending_gestures.push(TouchGesture::Pinch { center, scale, rotation });
    }
    
    /// Recognize swipe gesture
    fn recognize_swipe(&mut self, start_position: Vec2, end_position: Vec2, direction: SwipeDirection, velocity: f32) {
        self.pending_gestures.push(TouchGesture::Swipe { 
            start_position, 
            end_position, 
            direction, 
            velocity 
        });
        tracing::debug!("Recognized swipe {:?} with velocity {}", direction, velocity);
    }
    
    /// Get and clear pending gestures
    pub fn drain_gestures(&mut self) -> Vec<TouchGesture> {
        std::mem::take(&mut self.pending_gestures)
    }
    
    /// Get active touch count
    pub fn active_touch_count(&self) -> usize {
        self.active_touches.len()
    }
    
    /// Get active touches
    pub fn active_touches(&self) -> &HashMap<TouchId, TouchState> {
        &self.active_touches
    }
    
    /// Set active touch tool
    pub fn set_tool(&mut self, tool: TouchTool) {
        self.tools.active_tool = tool;
        tracing::info!("Touch tool changed to {:?}", tool);
    }
    
    /// Get current touch tool
    pub fn current_tool(&self) -> TouchTool {
        self.tools.active_tool
    }
    
    /// Check if position is within touch radius of any active touch
    pub fn is_position_touched(&self, position: Vec2) -> bool {
        self.active_touches.values()
            .any(|touch| touch.position.distance(position) <= touch.radius)
    }
    
    /// Update gesture recognition (called each frame)
    pub fn update(&mut self, _delta_time: f32) {
        // Handle timeout-based gesture recognition
        let now = Instant::now();
        
        // Handle single tap timeout for double-tap detection
        if let GestureState::PotentialDoubleTap { first_tap_time, first_tap_pos } = &self.gesture_state {
            if now.duration_since(*first_tap_time) > self.thresholds.double_tap_max_interval {
                self.recognize_tap(*first_tap_pos, 1);
                self.gesture_state = GestureState::Idle;
            }
        }
    }
}
