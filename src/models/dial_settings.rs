// Settings for dial configuration and visualization

/// Represents the configuration options for dial visualization
#[derive(Debug, Clone)]
pub struct DialSettings {
    pub show_degree_marks: bool,
    pub show_planet_labels: bool,
    pub show_midpoints: bool,
    pub line_thickness: f64,
    pub base_color: String,
}

impl Default for DialSettings {
    fn default() -> Self {
        Self {
            show_degree_marks: true,
            show_planet_labels: true,
            show_midpoints: true,
            line_thickness: 1.0,
            base_color: "#FFFFFF".to_string(),
        }
    }
}

impl DialSettings {
    /// Creates new settings with custom options
    pub fn new(
        show_degree_marks: bool,
        show_planet_labels: bool,
        show_midpoints: bool,
        line_thickness: f64,
        base_color: String,
    ) -> Self {
        Self {
            show_degree_marks,
            show_planet_labels,
            show_midpoints,
            line_thickness,
            base_color,
        }
    }
}
