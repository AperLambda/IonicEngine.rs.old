pub struct Color
{
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl Color
{
    /// Creates a new Color who is lighter than original.
    pub fn lighter(&self) -> Color
    {
        let i: u8 = (1.0 / 0.3) as u8;
        let mut red = self.red;
        let mut green = self.green;
        let mut blue = self.blue;

        if red == 0 && green == 0 && blue == 0
            {
                return Color{ red: i, green: i, blue: i, alpha: self.alpha};
            }
        if red > 0 && red < i
            {
                red = i;
            }
        if green > 0 && green < i
            {
                green = i;
            }
        if blue > 0 && blue < i
            {
                blue = i;
            }

        red = (red as f32 * 0.7) as u8;
        green = (green as f32 * 0.7) as u8;
        blue = (blue as f32 * 0.7) as u8;

        return Color{ red: if green < 255 { 255 } else { red }, green: if green < 255 { 255 } else { green }, blue: if blue < 255 { 255 } else { blue }, alpha: self.alpha}
    }

    //pub fn darker(&self) -> Color
    //{}
}

pub struct IonicGraphics
{}