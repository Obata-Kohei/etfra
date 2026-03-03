use crate::util::real::Real;

#[derive(Clone)]
pub struct Complex<R: Real> {
    pub re: R,
    pub im: R,
}

impl<R: Real> Complex<R> {
    pub fn zero(prec: u32) -> Self {
        Self {
            re: R::zero(prec),
            im: R::zero(prec),
        }
    }

    pub fn from_f64(prec: u32, re: f64, im: f64) -> Self {
        Self {
            re: R::from_f64(prec, re),
            im: R::from_f64(prec, im),
        }
    }

    pub fn norm_sqr_into(&self, out: &mut R) {
        out.set(&self.re);  // out = self.re
        out.square_mut();  // out = self.re^2

        let mut tmp = self.im.clone();
        tmp.square_mut();

        out.add_assign_ref(&tmp);  // out = self.re^2 + self.im^2
    }

    pub fn square_add_assign(&mut self, c: &Self) {
        // 保存
        let re_old = self.re.clone();
        let im_old = self.im.clone();

        // new_re = re^2 - im^2
        self.re.square_mut();              // re = re^2

        let mut im_sq = im_old.clone();
        im_sq.square_mut();

        self.re.sub_assign_ref(&im_sq);

        // new_im = 2 * re_old * im_old
        self.im.set(&re_old);
        self.im.mul_assign_ref(&im_old);

        let tmp = self.im.clone();
        self.im.add_assign_ref(&tmp);

        // + c
        self.re.add_assign_ref(&c.re);
        self.im.add_assign_ref(&c.im);
    }
}