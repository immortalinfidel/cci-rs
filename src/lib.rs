#![feature(external_doc)]

use md_rs::MD;
use typprice_rs::TypPrice;
use ta_common::traits::Indicator;



#[doc(include = "../README.md")]
pub struct CCI {
    md: MD,
    typ_price: TypPrice,

}


impl CCI {
    pub fn new(period: u32) -> CCI {
        Self {
            md: MD::new(period),
            typ_price: TypPrice::new(),

        }
    }
}

impl Indicator<[f64; 3], Option<f64>> for CCI {
    fn next(&mut self, input: [f64; 3]) -> Option<f64> {
        let tp = self.typ_price.next(input);

        let md = self.md.next(tp);

        return md.map(|val| {
            let cci = (tp - self.md.get_current_sma()) / (0.015 * val);
            cci
        });
    }

    fn reset(&mut self) {
        self.md.reset();
        self.typ_price.reset();
    }
}

#[cfg(test)]
mod tests {
    use crate::CCI;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut cci = CCI::new(5);
        assert_eq!(cci.next([82.15, 81.29, 81.59]), None);
        assert_eq!(cci.next([81.89, 80.64, 81.06]), None);
        assert_eq!(cci.next([83.03, 81.31, 82.87]), None);
        assert_eq!(cci.next([83.30, 82.65, 83.00]), None);
        assert_eq!(cci.next([83.85, 83.07, 83.61]), Some(105.01453488372036));
        assert_eq!(cci.next([83.90, 83.11, 83.15]), Some(64.23611111111184));
        assert_eq!(cci.next([83.33, 82.49, 82.84]), Some(-29.632609278624987));
        assert_eq!(cci.next([84.30, 82.30, 83.99]), Some(69.54436450839174));
        assert_eq!(cci.next([84.84, 84.15, 84.55]), Some(166.66666666667416));
        assert_eq!(cci.next([85.00, 84.11, 84.36]), Some(82.02011106108519));
        assert_eq!(cci.next([85.90, 84.03, 85.53]), Some(95.50079686159197));
        assert_eq!(cci.next([86.58, 85.39, 86.54]), Some(130.91226756520678));
        assert_eq!(cci.next([86.98, 85.76, 86.89]), Some(99.16327453640989));
        assert_eq!(cci.next([88.00, 87.17, 87.77]), Some(116.34153237206694));
        assert_eq!(cci.next([87.87, 87.01, 87.29]), Some(71.92795354899985));
    }
}
