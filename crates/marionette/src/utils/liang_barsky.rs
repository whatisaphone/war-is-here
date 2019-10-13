use crate::darksiders1::gfc;
use na::Point2;

// https://en.wikipedia.org/wiki/Liang–Barsky_algorithm
#[allow(clippy::similar_names)]
pub fn liang_barsky(
    rect: &gfc::TRect<f32>,
    p: Point2<f32>,
    q: Point2<f32>,
) -> Option<[Point2<f32>; 2]> {
    let xmin = rect.left;
    let xmax = rect.right;
    let ymin = rect.top;
    let ymax = rect.bottom;
    let x1 = p.x;
    let y1 = p.y;
    let x2 = q.x;
    let y2 = q.y;

    // defining variables
    let p1 = -(x2 - x1);
    let p2 = -p1;
    let p3 = -(y2 - y1);
    let p4 = -p3;

    let q1 = x1 - xmin;
    let q2 = xmax - x1;
    let q3 = y1 - ymin;
    let q4 = ymax - y1;

    let mut posarr = [0.0; 5];
    let mut negarr = [0.0; 5];
    let mut posind = 1;
    let mut negind = 1;
    posarr[0] = 1.0;
    negarr[0] = 0.0;

    if (p1 == 0.0 && q1 < 0.0) || (p3 == 0.0 && q3 < 0.0) {
        return None;
    }
    if p1 != 0.0 {
        let r1 = q1 / p1;
        let r2 = q2 / p2;
        if p1 < 0.0 {
            negarr[negind] = r1; // for negative p1, add it to negative array
            negind += 1;
            posarr[posind] = r2; // and add p2 to positive array
            posind += 1;
        } else {
            negarr[negind] = r2;
            negind += 1;
            posarr[posind] = r1;
            posind += 1;
        }
    }
    if p3 != 0.0 {
        let r3 = q3 / p3;
        let r4 = q4 / p4;
        if p3 < 0.0 {
            negarr[negind] = r3;
            negind += 1;
            posarr[posind] = r4;
            posind += 1;
        } else {
            negarr[negind] = r4;
            negind += 1;
            posarr[posind] = r3;
            posind += 1;
        }
    }

    let (xn1, yn1, xn2, yn2);
    let rn1 = negarr[..negind]
        .iter()
        .max_by(|x, y| PartialOrd::partial_cmp(x, y).unwrap())
        .unwrap(); // maximum of negative array
    let rn2 = posarr[..posind]
        .iter()
        .min_by(|x, y| PartialOrd::partial_cmp(x, y).unwrap())
        .unwrap(); // minimum of positive array

    if rn1 > rn2 {
        return None;
    }

    xn1 = x1 + p2 * rn1;
    yn1 = y1 + p4 * rn1; // computing new points

    xn2 = x1 + p2 * rn2;
    yn2 = y1 + p4 * rn2;

    Some([Point2::new(xn1, yn1), Point2::new(xn2, yn2)])
}
