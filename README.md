# automesh

[![stable](https://img.shields.io/badge/docs-stable-blue)](https://docs.rs/automesh)
[![dev](https://img.shields.io/badge/docs-latest-blue)](https://autotwin.github.io/automesh/automesh)
[![DOI](https://img.shields.io/badge/DOI-10.1234/zenodo.12345678-blue)](https://doi.org/10.1234/zenodo.12345678)

Automatic mesh generation

## Introduction

The current Autotwin workflow has the following broad steps:

1. Medical imaging
2. Segmentation
3. Mesh generation
4. Simulation
5. Injury risk assessment

As of 2024-06-24, the workflow has demonstrated *automation*, the ability to complete part of the workflow without human interaction, on over 100 patient medical image data sets for both the **segmentation** and **mesh generation** steps. The **simulation** and **injury risk** assessment steps, however, remain as future work.

Since inception of this project, the production of an open-source software work product has been a cornerstone philosophy and aspiration.

The workflow depends on amalgamation and orchestration of several pieces of software *prerequisites*.  One current prerequisite used for part of the mesh generation step is called Sculpt, which

* is closed-source,
* proprietary to SNL, and
* requires that void be meshed and included as part of the discretized geometric model.
  * Void inclusion can unnecessarily bloat the model.
  * For example, one recent proof-of-concept [exercise](https://github.com/autotwin/mesh/blob/main/doc/npy_to_mesh_part_3.md) using the IXI012-HH-1211-T1 data set showed that for a high-fidelity mesh created from segmented data, the void accounted for 2,389,783 elements (55%) of the total mesh 4,329,925 elements, with skull, cerebral spinal fluid, and brain, accounting for the remaining portions, 240,895 elements (6%), 448,654 elements (10%), and 1,250,593 elements (29%), respectively.

Elimination of the unnecessary void mesh is a top priority toward enhancement of mesh quality.  Additional mesh enchancement topics include smoothing and adaptivity.

Enhanced mesh quality can improve solver convergence rates, decrease overhead (memory footprint), and provide better overall geometric fidelity to the underlying human anatomy.

## Project Tasks

### Task 1: Solver Automation

*  **Mesh output decks**. Mesh outputs are solver inputs.  Mesh outputs must be automated to provide solver integration and automation for Sierra Solid Mechanics (SSM) in the Genesis/Exodus format, ABAQUS (.inp format), and Generic ASCII (e.g., .vtk), specific format to be determined.
*  **Solver input decks**.  Current solver runs have hard-coded and manual hand-tailored input decks.  This process must be rewritten and fully automated.

### Task 2: Injury-Risk Automation

* **Globalized measures**.  Current workflows (e.g., MPS, MPSR, 95th percentile cloud maps) will be rewritten to become standardized and flexible, enabling automation.
* **Localized measures**.  Current whole-brain visualization workflows will be formalized into repeatable and flexible software recipes, making manual “point-and-click” GUI steps unnecessary.

### Task 3: Open-Source Mesh Generation

* **Open-source**.  Reproduce Sculpt mesh generation functionality as an open-source software component.

### Task 4: Mesh Enhancements

* **Filtering**.  Process the mesh with high-frequency filtering (e.g., Taubin smoothing).
* **Adaptivity**.  Process the mesh to be adaptivity, refining in regions of interest and coarsening in regions where abundance of mesh resolution is unnecessary.

Reference: 2024-06-21-1218-EST-ONR-Statement-of-Work.pdf

## Specific next steps

A minimum working example (MWE) of the `letter F` model (see [https://github.com/autotwin/mesh/blob/main/doc/npy_to_mesh.md](https://github.com/autotwin/mesh/blob/main/doc/npy_to_mesh.md)) will be used as a unit test through the following specific next steps:

* Given:
  * Semantic segmentation (as a [`.spn`](https://github.com/autotwin/mesh/blob/main/tests/files/letter_f.spn) file)
  * Configuration recipe (as a [`.yml`](https://github.com/autotwin/mesh/blob/main/tests/files/letter_f_autotwin.yml) file)
* Create:
  * Rust command line application that outputs equivalent Sculpt outputs, without void as a mesh constituent, as
    * ABAQUS ascii mesh file (as a `.inp` file)
    * SSM-ready mesh file (as a `.e` file, Genesis/Exodus [NetCDF](https://www.unidata.ucar.edu/software/netcdf/) binary format)
    * ascii neutral mesh file (as a file type that is currently to be determined)
* Next steps:
  * Taubin smoothing (see [Taubin 1995](https://dl.acm.org/doi/pdf/10.1145/218380.218473) and [Chen 2010](https://link.springer.com/content/pdf/10.1007/s00707-009-0274-0.pdf))
  * Dualization
