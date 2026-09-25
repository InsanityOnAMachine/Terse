use ratatui::layout::Rect;

#[derive(Debug, Default, PartialEq)]
pub struct MinSize {
    pub width: u16,
    pub height: u16,
}

impl MinSize {
    pub fn new(width: u16, height: u16) -> Self {
        Self {width, height}
    }
    
    pub fn hmerge(sizes: Vec<MinSize>) -> Self {
        sizes.into_iter().reduce(|acc, item| MinSize::new(
            acc.width + item.width,
            acc.height.max(item.height),
        )).unwrap_or_default()
    }

    pub fn vmerge(sizes: Vec<MinSize>) -> Self {
        sizes.into_iter().reduce(|acc, item| MinSize::new(
            acc.width.max(item.width),
            acc.height + item.height,
        )).unwrap_or_default()
    }

    pub fn fits_in(&self, area: Rect) -> bool {
        self.width <= area.width && self.height <= area.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hmerge_works() {
        let a = MinSize::new( 
            2,5,
        );
        let b = MinSize::new( 
            8,3,
        );

        let merged = MinSize::hmerge(vec![a, b]);
        assert_eq!(merged, MinSize::new(
            10, 5,
        ));
    }

    #[test]
    fn vmerge_works() {
        let a = MinSize::new( 
            2,5,
        );
        let b = MinSize::new( 
            8,4,
        );

        let merged = MinSize::vmerge(vec![a, b]);
        assert_eq!(merged, MinSize::new(
            8, 9,
        ));
    }

    #[test]
    fn fits_in_works() {
        let s = MinSize::new(
            5,15,
        );
        
        assert!(s.fits_in(Rect::new(4,7,5,15)), "Testing that it fits in an equal rect");
        assert!(s.fits_in(Rect::new(4,7,9,17)), "Testing that it fits in a larger rect");
        assert!(!s.fits_in(Rect::new(4,7,3,15)), "Testing that it doesn't fit in a too thin rect");
        assert!(!s.fits_in(Rect::new(4,7,5,12)), "Testing that it doesn't fit in a too short rect");
    }
}
