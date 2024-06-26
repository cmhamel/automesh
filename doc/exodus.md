# Exodus II

## Excerpts from Schoof-1994[^Schoof-1994]:

"EXODUS II is a model developed to store and retrieve data for finite element analyses.  It is used for preprocesing (problem definition), postprocessing (results visualization), as well as code to code data transfer.  An EXODUS II data file is a random access, machine independent binary file..."

EXODUS II depends on the Network Common Data Form ([NetCDF](https://www.unidata.ucar.edu/software/netcdf/)) library.

NetCDF is a public domain database library that provides low-level data storage.  The NetCDF library stores data in eXternal Data Representation (XDR) format, which provides machine independency.

EXODUS II library functions provide a map between finite element data objects and NetCDF dimensions, attributes, and variables.

EXODUS II data objects:

* Initialization Data
  * Number of nodes
  * Number of elements
  * *optional* informational text
  * et cetera
* Model - static objects (i.e., objects that do not change over time)
  * Nodal coordinates
  * Element connectivity
  * Node sets
  * Side sets
* *optional* Results
  * Nodal results
  * Element results
  * Global results

Note: automesh will use Initialization Data and Model sections; it will not use the Results section.

Quadrilateral | Hexahedral 
:---: | :---:
![exodus_quad_node_numbering](fig/exodus_quad_node_numbering.png) | ![exodus_hex_node_numbering](fig/exodus_hex_node_numbering.png)

> Figure 1: EXODUS II node numbering scheme for quadrilateral and hexahedral finite elements.

Quadrilateral | Hexahedral 
:---: | :---:
![exodus_quad_sideset_numbering](fig/exodus_quad_sideset_numbering.png) | ![exodus_hex_sideset_numbering](fig/exodus_hex_sideset_numbering.png)

> Figure 2: EXODUS II sideset numbering scheme for quadrilateral and hexahedral finite elements.

## References

[^Schoof-1994]: Schoof LA, Yarberry VR. EXODUS II: a finite element data model. Sandia National Lab.(SNL-NM), Albuquerque, NM (United States); 1994 Sep 1. [link](https://www.osti.gov/biblio/10102115)
