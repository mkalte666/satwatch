use sgp4::{Classification, Elements};

pub fn element_copy(element: &Elements) -> Elements {
    let classification = match element.classification {
        Classification::Unclassified => Classification::Unclassified,
        Classification::Classified => Classification::Classified,
        Classification::Secret => Classification::Secret,
    };
    Elements {
        object_name: element.object_name.as_ref().cloned(),
        international_designator: element.international_designator.as_ref().cloned(),
        norad_id: element.norad_id,
        classification,
        datetime: element.datetime,
        mean_motion_dot: element.mean_motion_dot,
        mean_motion_ddot: element.mean_motion_ddot,
        drag_term: element.drag_term,
        element_set_number: element.element_set_number,
        inclination: element.inclination,
        right_ascension: element.right_ascension,
        eccentricity: element.eccentricity,
        argument_of_perigee: element.argument_of_perigee,
        mean_anomaly: element.mean_anomaly,
        mean_motion: element.mean_motion,
        revolution_number: element.revolution_number,
        ephemeris_type: element.ephemeris_type,
    }
}
