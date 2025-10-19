# What is Going on in this Repo?

This is a detailed, but not-too-technical explanation of the physics and computational techniques at work in this black hole simulation. It is intended for an intelligent, but general audience. Equations will be included for illustration and completeness, but you do not need to understand them to understand the discussion.

## Background

In general relativity, energy curves spacetime. If enough energy is crammed into a small enough region of space, the curvature becomes so extreme that not even light can escape, and a black hole is formed. In this project, we will be studying the evolution of a wave of energy under different conditions to learn about the physical properties of systems that lead to black hole formation.

The kind of energy we're going to be working with is called a "massless scalar field." Using the common analogy of spacetime as a big rubber sheet with planets and stars as balls rolling around on it, the field we're discussing would be like ripples in the rubber caused by a fist coming down and lightly tapping the sheet.

<div style="width: 600px; margin: 0 auto;">
    <figure>
        <img src="images/gravitational_wave.jpg" alt="A cartoon depicting a scalar field analogy" width="600">
        <figcaption style="text-align: left;">Figure 1: The rubber sheet is our spacetime. The weight of the planets warps it, and the scalar field is like someone hitting the sheet.</figcaption>
    </figure>
</div>

We've already established that a strong enough wave will form a black hole. This would be akin to the fist punching right through our rubber sheet. If the wave isn't strong enough to form a black hole, then the ripples will simply disperse outwards to infinity and nothing much happens. The question this project is concerned with is: **What happens if we confine our spacetime?** In other words, what happens to these small energy waves if they're not allowed to diffuse out to infinity?

## Motivation

Why on earth are we confining our spacetime like this? The answer has to do with a concept called "stability." **A spacetime is defined to be <u>un</u>stable if any amount of energy put into it, no matter how small, inevitably forms a black hole.** Stability simply means that there are configurations of the spacetime that *don't* collapse into black holes.

The flat universe that we live in is known to be stable (we said above that weak energy will simply diffuse to infinity), but there are more exotic flavours of spacetime whose stability is not known, and it is interesting to ask what changes from flatness can transition us from a stable spacetime to an unstable one. The point of this whole project is to provide evidence that confining flat spacetime is all it takes to make it unstable. This claimed instability then has implications in the study of more exotic forms of spacetime, which themselves have implications in string theory, but such discussions would take us way too far afield. All we need to concern ourselves with here is our little flat space model.

## The Problem Statement

We are trying to provide evidence that confinement takes otherwise stable flat space and makes it unstable. To do this, we will show that smaller and smaller energy packets that would not form black holes in regular flat space *will* form black holes in confined flat space.

How does this process work? Imagine our scalar field which initially does not have enough energy to collapse. It starts diffusing outwards, but eventually reaches our artificial boundary. The energy of our system must be conserved, so the field has no choice but to be reflected back towards the center. Naively, one might think that this causes a stable pattern of infinite back and forth reflection: inwards, outwards, inwards, outwards, etc., but remember that energy curves spacetime! Every time the energy collapses into the center, it comes under the influence of its own self-gravity, making the energy packet a little denser. As this process repeats over and over, the energy packet becomes denser and denser until it inevitably forms a black hole, thus demonstrating instability.

Now that we have a well-posed problem, we can talk about the physics of our system.

## The Physics
