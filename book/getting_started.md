# Getting Started

*Work in progress*

We start with a segmentation, created in Python, that describes an inner
domain with two outer layers.  The segmentation encodes

* `0` for void (or background), shown in gray,
* `1` for the inner domain, shown in green,
* `2` for the intermediate layer, shown in yellow, and
* `3` for the outer layer, shown in magenta.

A very coarse (`7 x 7 x 7`) segmentation, taken as a midline cut plane,
appears as follows:

<style>
    .container {
        display: flex; /* Use flexbox layout */
    }
    .grid {
        display: grid;
        grid-template-columns: repeat(7, 50px);
        grid-template-rows: repeat(7, 50px);
        gap: 1px;
    }
    .gridito {
        display: grid;
        grid-template-columns: repeat(7, 20px);
        grid-template-rows: repeat(7, 20px);
        gap: 1px;
    }
    .cell {
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 24px;
        color: white;
    }
    .zero {
        /* background-color: gray; */
        background-color: rgb(128, 128, 128);
    }
    .one {
        /* background-color: green; */
        background-color: rgb(0, 255, 0); /* RGB value for green */
        color: black;  /* text color */
    }
    .two {
        /* background-color: yellow; */
        background-color: rgb(255, 255, 0);
        color: black;  /* text color */
    }
    .three {
        /* background-color: magenta; */
        background-color: rgb(255, 0, 255);
    }
</style>

<div class="grid">
    <!--row 1-->
    <div class="cell zero">0</div>
    <div class="cell zero">0</div>
    <div class="cell zero">0</div>
    <div class="cell three">3</div>
    <div class="cell zero">0</div>
    <div class="cell zero">0</div>
    <div class="cell zero">0</div>
    <!--row 2-->
    <div class="cell zero">0</div>
    <div class="cell zero">0</div>
    <div class="cell three">3</div>
    <div class="cell two">2</div>
    <div class="cell three">3</div>
    <div class="cell zero">0</div>
    <div class="cell zero">0</div>
    <!--row 3-->
    <div class="cell zero">0</div>
    <div class="cell three">3</div>
    <div class="cell two">2</div>
    <div class="cell one">1</div>
    <div class="cell two">2</div>
    <div class="cell three">3</div>
    <div class="cell zero">0</div>
    <!--row 4-->
    <div class="cell three">3</div>
    <div class="cell two">2</div>
    <div class="cell one">1</div>
    <div class="cell one">1</div>
    <div class="cell one">1</div>
    <div class="cell two">2</div>
    <div class="cell three">3</div>
    <!--row 5-->
    <div class="cell zero">0</div>
    <div class="cell three">3</div>
    <div class="cell two">2</div>
    <div class="cell one">1</div>
    <div class="cell two">2</div>
    <div class="cell three">3</div>
    <div class="cell zero">0</div>
    <!--row 6-->
    <div class="cell zero">0</div>
    <div class="cell zero">0</div>
    <div class="cell three">3</div>
    <div class="cell two">2</div>
    <div class="cell three">3</div>
    <div class="cell zero">0</div>
    <div class="cell zero">0</div>
    <!--row 7-->
    <div class="cell zero">0</div>
    <div class="cell zero">0</div>
    <div class="cell zero">0</div>
    <div class="cell three">3</div>
    <div class="cell zero">0</div>
    <div class="cell zero">0</div>
    <div class="cell zero">0</div>
</div>

Consider each slice, `1` to `7`, in succession:

<div class="container">
    <!--slice 1-->
    1&nbsp;<div class="gridito">
        <!--row 1-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 2-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 3-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 4-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 5-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 6-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 7-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
    </div>
    &nbsp;
    &nbsp;
    &nbsp;
    <!--slice 2-->
    2&nbsp;<div class="gridito">
        <!--row 1-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 2-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 3-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 4-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 5-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 6-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 7-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
    </div>
    &nbsp;
    &nbsp;
    &nbsp;
    <!--slice 3-->
    3&nbsp;<div class="gridito">
        <!--row 1-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 2-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 3-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 4-->
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell one"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <!--row 5-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 6-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 7-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
    </div>
    &nbsp;
    &nbsp;
    &nbsp;
    <!--slice 4-->
    4&nbsp;<div class="gridito">
        <!--row 1-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 2-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 3-->
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell one"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <!--row 4-->
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell one"></div>
        <div class="cell one"></div>
        <div class="cell one"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <!--row 5-->
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell one"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <!--row 6-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 7-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
    </div>
</div>
&nbsp;
<div class="container">
    <!--slice 5-->
    5&nbsp;<div class="gridito">
        <!--row 1-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 2-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 3-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 4-->
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell one"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <!--row 5-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 6-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 7-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
    </div>
    &nbsp;
    &nbsp;
    &nbsp;
    <!--slice 6-->
    6&nbsp;<div class="gridito">
        <!--row 1-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 2-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 3-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 4-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell two"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 5-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 6-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 7-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
    </div>
    &nbsp;
    &nbsp;
    &nbsp;
    <!--slice 7-->
    7&nbsp;<div class="gridito">
        <!--row 1-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 2-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 3-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 4-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell three"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 5-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 6-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <!--row 7-->
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
        <div class="cell zero"></div>
    </div>
    &nbsp;
    &nbsp;
    &nbsp;
</div>

> Remark: The (`7 x 7 x 7`) segmentation can be thought of as a conceptual start point
for a process called
[Loop subdivision](https://en.wikipedia.org/wiki/Loop_subdivision_surface),
used to produce spherical shapes at higher resolutions.  
See [Octa Loop](https://github.com/autotwin/mesh/blob/main/doc/octa_loop.md) for additional information.
A sphere in resolutions of (`24 x 24 x 24`) and (`48 x 48 x 48`), used an example
in the [Sphere with Shells](https://autotwin.github.io/automesh/analysis/sphere_with_shells/index.html) section,
is shown below:

![spheres_cont_cut](analysis/sphere_with_shells/img/spheres_cont_cut.png)

