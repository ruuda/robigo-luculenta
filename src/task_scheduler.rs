// Robigo Luculenta -- Proof of concept spectral path tracer in Rust
// Copyright (C) 2014-2015 Ruud van Asseldonk
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::cmp::max;
use std::collections::vec_deque::RingBuf;
use std::iter::AdditiveIterator;
use std::num::Float;
use std::time::Duration;
use time::{Timespec, get_time};
use gather_unit::GatherUnit;
use plot_unit::PlotUnit;
use pop_iter::PopFrontIter;
use tonemap_unit::TonemapUnit;
use trace_unit::TraceUnit;

pub enum Task {
    /// Do nothing, wait a while.
    Sleep,

    /// Trace a certain number of rays and store the mapped photons.
    Trace(Box<TraceUnit>),

    /// Plot all intermediate mapped photons to a canvas of CIE XYZ values.
    Plot(Box<PlotUnit>, Vec<Box<TraceUnit>>),

    /// Combine all CIE XYZ canvases and accumulate them into the final image.
    Gather(Box<GatherUnit>, Vec<Box<PlotUnit>>),

    /// Convert the CIE XYZ values to sRGB and display the image.
    Tonemap(Box<TonemapUnit>, Box<GatherUnit>)
}

/// Tonemap every 30 seconds.
fn tonemap_interval() -> Duration {
    Duration::seconds(30)
}

/// Handles splitting the workload across threads.
pub struct TaskScheduler {
    /// The number of completed trace batches. Used to measure performance.
    traces_completed: u32,

    /// Previous measurements of batches/second, used to determine variance.
    performance: RingBuf<f32>,

    /// The number of trace units to use. Not all of them have to be
    /// active simultaneously.
    number_of_trace_units: usize,

    /// The trace units which are available for tracing rays.
    available_trace_units: RingBuf<Box<TraceUnit>>,

    /// The trace units which have mapped photons that must be plotted,
    /// before the trace unit can be used again.
    done_trace_units: RingBuf<Box<TraceUnit>>,

    /// The plot units which are available for plotting mapped photons.
    available_plot_units: RingBuf<Box<PlotUnit>>,

    /// The plot units which have a screen that must be accumulated
    /// before the plot unit can be used again.
    done_plot_units: RingBuf<Box<PlotUnit>>,

    /// The gather unit, when it is available.
    gather_unit: Option<Box<GatherUnit>>,

    /// The tonemap unit, when it is available.
    tonemap_unit: Option<Box<TonemapUnit>>,

    /// The last time the image was tonemapped (and displayed).
    last_tonemap_time: Timespec,

    /// Whether a new gather task has been executed since the last
    /// tonemapping task was executed.
    image_changed: bool
}

impl TaskScheduler {
    /// Creates a new task scheduler, that will render `scene` to a
    /// canvas of the specified size, using `concurrency` threads.
    pub fn new(concurrency: usize, width: u32, height: u32) -> TaskScheduler {
        // More trace units than threads seems sensible,
        // but less plot units is acceptable,
        // because one plot unit can handle multiple trace units.
        let n_trace_units = concurrency * 3;
        let n_plot_units = max(1us, concurrency / 2);

        // Build the trace units.
        let trace_units = (0 .. n_trace_units)
        .map(|i| { Box::new(TraceUnit::new(i, width, height)) })
        .collect::<RingBuf<Box<TraceUnit>>>();

        // Then build the plot units.
        let plot_units = (0 .. n_plot_units)
        .map(|i| { Box::new(PlotUnit::new(i, width, height)) })
        .collect::<RingBuf<Box<PlotUnit>>>();

        // There must be one gather unit and one tonemap unit.
        let gather_unit = Some(Box::new(GatherUnit::new(width, height)));
        let tonemap_unit = Some(Box::new(TonemapUnit::new(width, height)));

        TaskScheduler {
            traces_completed: 0,
            performance: RingBuf::new(),
            number_of_trace_units: n_trace_units,
            available_trace_units: trace_units,
            done_trace_units: RingBuf::new(),
            available_plot_units: plot_units,
            done_plot_units: RingBuf::new(),
            gather_unit: gather_unit,
            tonemap_unit: tonemap_unit,
            last_tonemap_time: get_time(),
            image_changed: false
        }
    }

    pub fn get_new_task(&mut self, completed_task: Task) -> Task {
        // Make the units that were used by the completed task available again.
        self.complete_task(completed_task);

        // If the last tonemapping time was more than x seconds ago,
        // an update should be done.
        let now = get_time();
        if now - self.last_tonemap_time > tonemap_interval() {
            // If the image has changed since it was last tonemapped,
            // tonemap it now.
            if self.image_changed {
                // Tonemapping can only be done if no gathering
                // and tonemapping are busy.
                if self.gather_unit.is_some() && self.tonemap_unit.is_some() {
                    return self.create_tonemap_task();
                }
            } else {
                // Otherwise, the plots must first be gathered, tonemapping
                // will happen once that is done.
                if self.gather_unit.is_some() &&
                   !self.done_plot_units.is_empty() {
                    return self.create_gather_task();
                }
            }
        }

        // If a substantial number of trace units is done, plot them first
        // so they can be recycled soon.
        if self.done_trace_units.len() > self.number_of_trace_units / 2 &&
            !self.available_plot_units.is_empty() {
            return self.create_plot_task();
        }

        // Then, if there are enough trace units available, go trace some rays!
        if !self.available_trace_units.is_empty() {
            return self.create_trace_task();
        }

        // Otherwise, some trace units need to be plotted to make them
        // available again.
        if !self.available_plot_units.is_empty() &&
           !self.done_trace_units.is_empty() {
            return self.create_plot_task();
        }

        // If no plot units are available (or all trace units are busy,
        // which should be impossible), gather some plots to make the plot
        // units available again.
        if self.gather_unit.is_some() && !self.done_plot_units.is_empty() {
            return self.create_gather_task();
        }

        // If everything is locked in dependencies and everything is a big
        // mess, simply wait a while for units to become available.
        Task::Sleep
    }

    fn create_trace_task(&mut self) -> Task {
        // Pick the first available trace unit, and use it for the task.
        // We know a unit is available, because this method would not
        // have been called otherwise.
        let trace_unit = self.available_trace_units.pop_front().unwrap();
        Task::Trace(trace_unit)
    }

    fn create_plot_task(&mut self) -> Task {
        // Pick the first available plot unit, and use it for the task.
        // We know a unit is available, because this method would not
        // have been called otherwise.
        let plot_unit = self.available_plot_units.pop_front().unwrap();

        // Take around half of the trace units which are done for this task.
        let done = self.done_trace_units.len();
        let n = max(1, done / 2);

        // Have it plot the trace units which are done.
        let trace_units: Vec<Box<TraceUnit>> = self.done_trace_units
        .pop_front_iter().take(n).collect();

        Task::Plot(plot_unit, trace_units)
    }

    fn create_gather_task(&mut self) -> Task {
        // We know the gather unit is available, because this method would
        // not have been called otherwise.
        let gather_unit = self.gather_unit.take().unwrap();

        // Have it gather all plot units which are done.
        let plot_units: Vec<Box<PlotUnit>> = self.done_plot_units
        .pop_front_iter().collect();

        Task::Gather(gather_unit, plot_units)
    }

    fn create_tonemap_task(&mut self) -> Task {
        // We know the units are available, because this method would
        // not have been called otherwise.
        let gather_unit = self.gather_unit.take().unwrap();
        let tonemap_unit = self.tonemap_unit.take().unwrap();

        Task::Tonemap(tonemap_unit, gather_unit)
    }

    /// Makes resources used by the task available again.
    fn complete_task(&mut self, task: Task) {
        match task {
            Task::Sleep => { },
            Task::Trace(unit) => self.complete_trace_task(unit),
            Task::Plot(unit, units) => self.complete_plot_task(unit, units),
            Task::Gather(unit, units) => self.complete_gather_task(unit, units),
            Task::Tonemap(t_unt, g_unt) => self.complete_tonemap_task(t_unt, g_unt)
        }
    }

    fn complete_trace_task(&mut self, trace_unit: Box<TraceUnit>) {
        println!("done tracing with unit {}", trace_unit.id);

        // The trace unit used for the task, now needs plotting before
        // it is available again.
        self.done_trace_units.push_back(trace_unit);

        // Keep statatistics about performance.
        self.traces_completed += 1;
    }

    fn complete_plot_task(&mut self,
                          plot_unit: Box<PlotUnit>,
                          trace_units: Vec<Box<TraceUnit>>) {
        println!("done plotting with unit {}", plot_unit.id);
        print!("the following trace units are available again: ");

        // All trace units that were plotted, can be used again now.
        for trace_unit in trace_units.into_iter() {
            print!(" {} ", trace_unit.id);
            self.available_trace_units.push_back(trace_unit);
        }

        println!("");

        // And the plot unit that was used, needs to be gathered before
        // it can be used again.
        self.done_plot_units.push_back(plot_unit);
    }

    fn complete_gather_task(&mut self,
                            gather_unit: Box<GatherUnit>,
                            plot_units: Vec<Box<PlotUnit>>) {
        println!("done gathering");
        print!("the following plot units are available again: ");

        // All plot units that were gathered, can be used again now.
        for plot_unit in plot_units.into_iter() {
            print!(" {} ", plot_unit.id);
            self.available_plot_units.push_back(plot_unit);
        }

        println!("");

        // The gather unit can now be used again as well.
        self.gather_unit = Some(gather_unit);

        // The image must have changed because of gathering.
        self.image_changed = true;
    }

    fn complete_tonemap_task(&mut self,
                             tonemap_unit: Box<TonemapUnit>,
                             gather_unit: Box<GatherUnit>) {
        println!("done tonemapping");

        // The tonemapper needed the gather unit,
        // so the gather unit is free now.
        self.gather_unit = Some(gather_unit);

        // And of course the tonemap unit itself is available again.
        self.tonemap_unit = Some(tonemap_unit);

        // The image is tonemapped now, so until a new gathering happens,
        // it will not change.
        self.image_changed = false;

        // Measure how many rays per seconds the renderer can handle.
        let now = get_time();
        let render_time = now - self.last_tonemap_time;
        let batches_per_sec = self.traces_completed as f32 * 1000.0 /
                              render_time.num_milliseconds() as f32;
        self.last_tonemap_time = now;
        self.traces_completed = 0;

        // Store the latest 512 measurements (should be about 4.25 hours).
        self.performance.push_back(batches_per_sec);
        if self.performance.len() > 512 { self.performance.pop_front(); }
        let n = self.performance.len() as f32;

        let mean = self.performance.iter().map(|&x| x).sum() / n;
        let sqr_mean = self.performance.iter().map(|&x| x * x).sum() / n;
        let variance = sqr_mean - mean * mean;

        println!("performance: {} +- {} batches/sec", mean, variance.sqrt());
    }
}
