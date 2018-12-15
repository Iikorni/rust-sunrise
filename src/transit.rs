// The MIT License (MIT)
//
// Copyright (c) 2018 Nathan Osman
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use std::f64;

const DEGREE: f64 = f64::consts::PI / 180.;

/// Calculates the Julian day for the local true solar transit.
pub fn solar_transit(day: f64, solar_anomaly: f64, ecliptic_longitude: f64) -> f64 {
    day + (0.0053 * f64::sin(solar_anomaly * DEGREE)
        - 0.0069 * f64::sin(2. * ecliptic_longitude * DEGREE))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    #[test]
    fn test_prime_meridian() {
        assert_relative_eq!(
            super::solar_transit(2440588., 358.30683, 281.08372),
            2440588.00245,
            epsilon = 0.00001
        )
    }
}
