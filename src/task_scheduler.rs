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

/// Handles splitting the workload across threads.
struct TaskScheduler<'s> {
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
}
