use crate::canvas::path::{arc, Arc, Path};

use iced_native::{Point, Size};
use lyon::path::builder::{Build, FlatPathBuilder, PathBuilder, SvgBuilder};

/// A [`Path`] builder.
///
/// Once a [`Path`] is built, it can no longer be mutated.
///
/// [`Path`]: struct.Path.html
#[allow(missing_debug_implementations)]
pub struct Builder {
    raw: lyon::path::builder::SvgPathBuilder<lyon::path::Builder>,
}

impl Builder {
    /// Creates a new [`Builder`].
    ///
    /// [`Builder`]: struct.Builder.html
    pub fn new() -> Builder {
        Builder {
            raw: lyon::path::Path::builder().with_svg(),
        }
    }

    /// Moves the starting point of a new sub-path to the given `Point`.
    #[inline]
    pub fn move_to(&mut self, point: Point) {
        let _ = self.raw.move_to(lyon::math::Point::new(point.x, point.y));
    }

    /// Connects the last point in the [`Path`] to the given `Point` with a
    /// straight line.
    ///
    /// [`Path`]: struct.Path.html
    #[inline]
    pub fn line_to(&mut self, point: Point) {
        let _ = self.raw.line_to(lyon::math::Point::new(point.x, point.y));
    }

    /// Adds an [`Arc`] to the [`Path`] from `start_angle` to `end_angle` in
    /// a clockwise direction.
    ///
    /// [`Arc`]: struct.Arc.html
    /// [`Path`]: struct.Path.html
    #[inline]
    pub fn arc(&mut self, arc: Arc) {
        self.ellipse(arc.into());
    }

    /// Adds a circular arc to the [`Path`] with the given control points and
    /// radius.
    ///
    /// The arc is connected to the previous point by a straight line, if
    /// necessary.
    ///
    /// [`Path`]: struct.Path.html
    pub fn arc_to(&mut self, a: Point, b: Point, radius: f32) {
        use lyon::{math, path};

        let a = math::Point::new(a.x, a.y);

        if self.raw.current_position() != a {
            let _ = self.raw.line_to(a);
        }

        let _ = self.raw.arc_to(
            math::Vector::new(radius, radius),
            math::Angle::radians(0.0),
            path::ArcFlags::default(),
            math::Point::new(b.x, b.y),
        );
    }

    /// Adds an [`Ellipse`] to the [`Path`] using a clockwise direction.
    ///
    /// [`Ellipse`]: struct.Arc.html
    /// [`Path`]: struct.Path.html
    pub fn ellipse(&mut self, arc: arc::Elliptical) {
        use lyon::{geom, math};

        let arc = geom::Arc {
            center: math::Point::new(arc.center.x, arc.center.y),
            radii: math::Vector::new(arc.radii.x, arc.radii.y),
            x_rotation: math::Angle::radians(arc.rotation),
            start_angle: math::Angle::radians(arc.start_angle),
            sweep_angle: math::Angle::radians(arc.end_angle),
        };

        let _ = self.raw.move_to(arc.sample(0.0));

        arc.for_each_quadratic_bezier(&mut |curve| {
            let _ = self.raw.quadratic_bezier_to(curve.ctrl, curve.to);
        });
    }

    /// Adds a cubic B??zier curve to the [`Path`] given its two control points
    /// and its end point.
    ///
    /// [`Path`]: struct.Path.html
    #[inline]
    pub fn bezier_curve_to(
        &mut self,
        control_a: Point,
        control_b: Point,
        to: Point,
    ) {
        use lyon::math;

        let _ = self.raw.cubic_bezier_to(
            math::Point::new(control_a.x, control_a.y),
            math::Point::new(control_b.x, control_b.y),
            math::Point::new(to.x, to.y),
        );
    }

    /// Adds a quadratic B??zier curve to the [`Path`] given its control point
    /// and its end point.
    ///
    /// [`Path`]: struct.Path.html
    #[inline]
    pub fn quadratic_curve_to(&mut self, control: Point, to: Point) {
        use lyon::math;

        let _ = self.raw.quadratic_bezier_to(
            math::Point::new(control.x, control.y),
            math::Point::new(to.x, to.y),
        );
    }

    /// Adds a rectangle to the [`Path`] given its top-left corner coordinate
    /// and its `Size`.
    ///
    /// [`Path`]: struct.Path.html
    #[inline]
    pub fn rectangle(&mut self, top_left: Point, size: Size) {
        self.move_to(top_left);
        self.line_to(Point::new(top_left.x + size.width, top_left.y));
        self.line_to(Point::new(
            top_left.x + size.width,
            top_left.y + size.height,
        ));
        self.line_to(Point::new(top_left.x, top_left.y + size.height));
        self.close();
    }

    /// Adds a circle to the [`Path`] given its center coordinate and its
    /// radius.
    ///
    /// [`Path`]: struct.Path.html
    #[inline]
    pub fn circle(&mut self, center: Point, radius: f32) {
        self.arc(Arc {
            center,
            radius,
            start_angle: 0.0,
            end_angle: 2.0 * std::f32::consts::PI,
        });
    }

    /// Closes the current sub-path in the [`Path`] with a straight line to
    /// the starting point.
    ///
    /// [`Path`]: struct.Path.html
    #[inline]
    pub fn close(&mut self) {
        self.raw.close()
    }

    /// Builds the [`Path`] of this [`Builder`].
    ///
    /// [`Path`]: struct.Path.html
    /// [`Builder`]: struct.Builder.html
    #[inline]
    pub fn build(self) -> Path {
        Path {
            raw: self.raw.build(),
        }
    }
}
