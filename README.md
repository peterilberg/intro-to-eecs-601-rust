# Overview

The code and the projects in this repository are based on the material for
MIT's course [EE 6.01 (2011) "Introduction to Electrical Engineering and
Computer Science I"](https://ocw.mit.edu/courses/6-01sc-introduction-to-electrical-engineering-and-computer-science-i-spring-2011/).

The course explains concepts such as state machines and probability with
accompanying Python code and exercises. I have reimplemented some of the
projects in Rust as an exercise.

## Building

    cargo build
    cargo test

## Use

Look at the tests and the example programs:

    cargo run --example accumulator
    cargo run --example stochastic
    cargo run --example state_estimator
