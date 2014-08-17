// Robigo Luculenta -- Proof of concept spectral path tracer in Rust
// Copyright (C) 2014 Ruud van Asseldonk
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
use std::collections::RingBuf;
use time::{Timespec, get_time};
use gather_unit::GatherUnit;
use plot_unit::PlotUnit;
use scene::Scene;
use tonemap_unit::TonemapUnit;
use trace_unit::TraceUnit;

pub enum Task<'s> {
    /// Do nothing, wait a while.
    Sleep,

    /// Trace a certain number of rays and store the mapped photons.
    Trace(Box<TraceUnit<'s>>),

    /// Plot all intermediate mapped photons to a canvas of CIE XYZ values.
    Plot(Box<PlotUnit>, Vec<Box<TraceUnit<'s>>>),

    /// Combine all CIE XYZ canvases and accumulate them into the final image.
    Gather(Box<GatherUnit>, Vec<Box<PlotUnit>>),

    /// Convert the CIE XYZ values to sRGB and display the image.
    Tonemap(Box<TonemapUnit>, Box<GatherUnit>)
}

/// Handles splitting the workload across threads.
pub struct TaskScheduler<'s> {
    /// The number of trace units to use. Not all of them have to be
    /// active simultaneously.
    number_of_trace_units: uint,

    /// The trace units which are available for tracing rays.
    available_trace_units: RingBuf<Box<TraceUnit<'s>>>,

    /// The trace units which have mapped photons that must be plotted,
    /// before the trace unit can be used again.
    done_trace_units: RingBuf<Box<TraceUnit<'s>>>,

    /// The number of plot units to use. Not all of them have to be
    /// active simultaneously.
    number_of_plot_units: uint,

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

impl<'s> TaskScheduler<'s> {
    /// Creates a new task scheduler, that will render `scene` to a
    /// canvas of the specified size, using `concurrency` threads.
    pub fn new<'sc>(concurrency: uint,
                    width: uint,
                    height: uint,
                    scene: &'sc Scene)
                    -> TaskScheduler<'sc> {
        // More trace units than threads seems sensible,
        // but less plot units is acceptable,
        // because one plot unit can handle multiple trace units.
        let n_trace_units = concurrency * 3;
        let n_plot_units = max(1u, concurrency / 2);

        // Build the trace units.
        let trace_units = range(0, n_trace_units)
        .map(|_| { box TraceUnit::new(scene, width, height) })
        .collect::<RingBuf<Box<TraceUnit>>>();

        // Then build the plot units.
        let plot_units = range(0, n_plot_units)
        .map(|_| { box PlotUnit::new(width, height) })
        .collect::<RingBuf<Box<PlotUnit>>>();

        // There must be one gather unit and one tonemap unit.
        let gather_unit = Some(box GatherUnit::new(width, height));
        let tonemap_unit = Some(box TonemapUnit::new(width, height));

        TaskScheduler {
            number_of_trace_units: n_trace_units,
            available_trace_units: trace_units,
            done_trace_units: RingBuf::new(),
            number_of_plot_units: n_plot_units,
            available_plot_units: plot_units,
            done_plot_units: RingBuf::new(),
            gather_unit: gather_unit,
            tonemap_unit: tonemap_unit,
            last_tonemap_time: get_time(),
            image_changed: false
        }
    }

    /// Makes resources used by the task available again.
    fn complete_task(&mut self, task: Task<'s>) {
        match task {
            Sleep => { },
            Trace(unit) => self.complete_trace_task(unit),
            Plot(unit, units) => self.complete_plot_task(unit, units),
            Gather(unit, units) => self.complete_gather_task(unit, units),
            Tonemap(t_unt, g_unt) => self.complete_tonemap_task(t_unt, g_unt)
        }
    }

    fn complete_trace_task(&mut self, trace_unit: Box<TraceUnit<'s>>) {
        println!("done tracing with unit x."); // TODO: unit numbers.

        // The trace unit used for the task, now needs plotting before
        // it is available again.
        self.done_trace_units.push(trace_unit);
    }

    fn complete_plot_task(&mut self,
                          plot_unit: Box<PlotUnit>,
                          trace_units: Vec<Box<TraceUnit<'s>>>) {
        println!("done plotting with unit x."); // TODO: unit numbers.
        print!("the following trace units are available again: ");

        // All trace units that were plotted, can be used again now.
        for trace_unit in trace_units.move_iter() {
            self.available_trace_units.push(trace_unit);
            print!(" x "); // TODO: unit numbers.
        }

        println!("");

        // And the plot unit that was used, needs to be gathered before
        // it can be used again.
        self.done_plot_units.push(plot_unit);
    }

    fn complete_gather_task(&mut self,
                            gather_unit: Box<GatherUnit>,
                            plot_units: Vec<Box<PlotUnit>>) {
        println!("done gathering");
        print!("the following plot units are available again: ");

        // All plot units that were gathered, can be used again now.
        for plot_unit in plot_units.move_iter() {
            self.available_plot_units.push(plot_unit);
            print!(" x "); // TODO: unit numbers.
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
        self.last_tonemap_time = get_time();
    }
}
