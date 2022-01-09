use sgp4::{Classification, Elements};

use crate::coordinates::*;
use crate::timebase::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use std::ops::Sub;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

use log::{error, warn};

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

pub struct ElementUpdate {
    pub id: u64,
    pub state: StateVector,
    pub orbit_points: Option<Vec<Coordinate>>,
}

pub struct ElementEngine {
    element_rx: Receiver<ElementUpdate>,
    add_tx: Sender<Elements>,
    remove_tx: Sender<u64>,
    timebase_tx: Sender<Timebase>,
}

struct WorkerData {
    elements: HashMap<u64, Elements>,
    timebase: Timebase,
    element_tx: Sender<ElementUpdate>,
    add_rx: Receiver<Elements>,
    remove_rx: Receiver<u64>,
    timebase_rx: Receiver<Timebase>,
}

impl ElementEngine {
    pub fn new() -> Self {
        let (element_tx, element_rx) = channel();
        let (add_tx, add_rx) = channel();
        let (remove_tx, remove_rx) = channel();
        let (timebase_tx, timebase_rx) = channel();
        thread::spawn(|| {
            let mut data = WorkerData {
                elements: HashMap::new(),
                timebase: Timebase::new(),
                element_tx,
                add_rx,
                remove_rx,
                timebase_rx,
            };
            data.run();
        });

        Self {
            element_rx,
            add_tx,
            remove_tx,
            timebase_tx,
        }
    }

    pub fn update_timebase(&self, timebase: Timebase) {
        self.timebase_tx.send(timebase).unwrap();
    }

    pub fn add(&self, element: &Elements) {
        self.add_tx.send(element_copy(element)).unwrap();
    }

    pub fn remove(&self, element: u64) {
        self.remove_tx.send(element).unwrap();
    }

    pub fn get_more(&self) -> Option<ElementUpdate> {
        match self.element_rx.try_recv() {
            Ok(e) => Some(e),
            Err(_) => None,
        }
    }
}

impl WorkerData {
    fn run(&mut self) {
        loop {
            if !self.wait_for_new_timebase() {
                break;
            }

            self.process_adds();
            self.process_removes();
            self.update();
        }
    }

    fn wait_for_new_timebase(&mut self) -> bool {
        let mut got_any_or_err = false;
        'wait_loop: loop {
            match self.timebase_rx.try_recv() {
                Ok(timebase) => {
                    self.timebase = timebase;
                    got_any_or_err = true;
                }
                Err(TryRecvError::Empty) => {
                    if !got_any_or_err {
                        std::thread::sleep(Duration::from_millis(10));
                    } else {
                        break 'wait_loop;
                    }
                }
                Err(_) => {
                    got_any_or_err = false;
                    break 'wait_loop;
                }
            }
        }

        got_any_or_err
    }

    fn process_adds(&mut self) {
        'add_loop: loop {
            match self.add_rx.try_recv() {
                Ok(elements) => {
                    self.elements.insert(elements.norad_id, elements);
                }
                Err(_) => {
                    break 'add_loop;
                }
            }
        }
    }

    fn process_removes(&mut self) {
        'remove_loop: loop {
            match self.remove_rx.try_recv() {
                Ok(elements) => {
                    self.elements.remove(&elements);
                }
                Err(_) => {
                    break 'remove_loop;
                }
            }
        }
    }

    fn update(&mut self) {
        let start = Instant::now();
        for (id, element) in &self.elements {
            let tle_epoch: DateTime<Utc> = DateTime::from_utc(element.datetime, Utc);
            let minutes = self.timebase.now_minutes(tle_epoch);
            if let Ok(constants) = sgp4::Constants::from_elements(element) {
                if let Ok(prediction) = constants.propagate(minutes) {
                    let mut state = StateVector::from(prediction);
                    state.coordinate.time = self.timebase.now_j2000_minutes();
                    //let mut orb_points : Vec<Coordinate> = Vec::new();
                    // miuntes/orbit = 1/(orbits/day)/24/60
                    /*let min_per_orbit = 1.0/element.mean_motion * 24.0 * 60.0;
                    let phase = min_per_orbit/24.0;
                    for i in 0..24 {
                        let o_pred = constants.propagate(minutes + phase * i as f64).unwrap();
                        let state = StateVector::from(o_pred);
                        orb_points.push(state.coordinate);
                    }*/
                    if let Err(e) = self.element_tx.send(ElementUpdate {
                        id: *id,
                        state,
                        orbit_points: None,
                    }) {
                        error!(
                            "Something is going very wrong the the TLE Simulation thread: {}",
                            e
                        );
                    }
                }
            }
        }
        let end = Instant::now();
        let sim_time = end.sub(start);
        if sim_time > Duration::from_secs_f64(1.0 / 60.0) {
            warn!("Sim took to long. {}ms", sim_time.as_millis())
        }
    }
}
