use bevy::prelude::*;

/// Component for the weapon display (floating weapon near camera)
#[derive(Component)]
pub struct WeaponDisplay {
    /// Timer for bob animation (up/down movement)
    pub bob_timer: f32,
    
    /// Speed of rotation animation
    pub rotation_speed: f32,
    
    /// Cooldown after shooting (weapon is forming)
    pub cooldown: Timer,
    
    /// Is the weapon currently forming/fading in?
    pub forming: bool,
    
    /// Base position offset from camera
    pub base_position: Vec3,
}

impl Default for WeaponDisplay {
    fn default() -> Self {
        Self {
            bob_timer: 0.0,
            rotation_speed: 1.0,  // radians per second
            cooldown: Timer::from_seconds(0.5, TimerMode::Once),
            forming: false,
            base_position: Vec3::new(1.5, -0.5, -2.0),  // Right, slightly down, in front
        }
    }
}

impl WeaponDisplay {
    /// Start cooldown after shooting
    pub fn trigger_cooldown(&mut self) {
        self.cooldown.reset();
        self.forming = true;
    }
    
    /// Check if weapon is ready to shoot
    pub fn is_ready(&self) -> bool {
        self.cooldown.finished()
    }
}

/// Event fired when player shoots
#[derive(Event)]
pub struct WeaponFiredEvent;
