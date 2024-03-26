use crate::types::*;

impl Into<geo::Point> for Point {
    fn into(self) -> geo::Point {
        geo::Point::new(self.x, self.y)
    }
}

impl Into<geo::Coord> for Point {
    fn into(self) -> geo::Coord {
        geo::coord!(x: self.x, y: self.y)
    }
}

impl Into<geo::MultiPoint> for MultiPoint<Point> {
    fn into(self) -> geo::MultiPoint {
        let points = self.points.into_iter().map(Into::into).collect();
        geo::MultiPoint::new(points)
    }
}

impl Into<geo::LineString> for LineString<Point> {
    fn into(self) -> geo::LineString {
        let coords = self.points.into_iter().map(Into::into).collect();
        geo::LineString::new(coords)
    }
}

impl Into<geo::MultiLineString> for MultiLineString<Point> {
    fn into(self) -> geo::MultiLineString {
        let lines = self.lines.into_iter().map(Into::into).collect();
        geo::MultiLineString::new(lines)
    }
}

impl Into<geo::Polygon> for Polygon<Point> {
    fn into(self) -> geo::Polygon {
        let mut rings: Vec<geo::LineString> = self
            .rings
            .into_iter()
            .map(|pv| pv.into_iter().map(Into::into).collect())
            .map(geo::LineString::new)
            .collect();
        let outer = rings.remove(0);
        geo::Polygon::new(outer, rings)
    }
}

impl Into<geo::MultiPolygon> for MultiPolygon<Point> {
    fn into(self) -> geo::MultiPolygon {
        let polygons = self.polygons.into_iter().map(Into::into).collect();
        geo::MultiPolygon::new(polygons)
    }
}

impl Into<Point> for geo::Point {
    fn into(self) -> Point {
        Point::new(self.x(), self.y(), None)
    }
}

impl Into<Point> for geo::Coord {
    fn into(self) -> Point {
        Point::new(self.x, self.y, None)
    }
}

impl Into<LineString<Point>> for geo::LineString {
    fn into(self) -> LineString<Point> {
        let mut linestring = LineString::new(None);
        linestring.add_points(self.points().map(Into::into));
        linestring
    }
}

impl Into<MultiLineString<Point>> for geo::MultiLineString {
    fn into(self) -> MultiLineString<Point> {
        let mut multilinestring = MultiLineString::new(None);
        for linestring in self.0 {
            multilinestring.add_line_with_cap(linestring.0.len());
            multilinestring.add_points(linestring.0.into_iter().map(Into::into));
        }
        multilinestring
    }
}
