Robigo Luculenta
================

A proof of concept spectral path tracer in Rust.

[![Build Status][ci-img]][ci]

From the Latin ‘luculentus’, meaning ‘bright’, ‘shining’, ‘impressive’,
‘gorgeous’ and ‘robigo’, meaning ‘rust’ (on metals, as well as the fungus).
This is a port of the proof of concept spectral path tracer
[Luculentus][luculentus] to the Rust programming language, released under
the [GNU General Public License][gplv3].

Robigo Luculenta traces rays at different wavelengths, giving it the ability
to simulate effects like dispersion and chromatic aberration. It was written
with code clarity as the primary goal; it is not optimised for speed, although
it is multithreaded.

[ci-img]:     https://travis-ci.org/ruud-v-a/robigo-luculenta.svg
[ci]:         https://travis-ci.org/ruud-v-a/robigo-luculenta
[luculentus]: https://github.com/ruud-v-a/luculentus
[gplv3]:      https://www.gnu.org/licenses/gpl.html
