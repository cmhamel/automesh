# Getting Started

*Work in progress*

We start with a segmentation, created in Python, that describes a sphere with
two concentric shells.  The segmentation encodes

* `0` for void (or background), shown in gray,
* `1` for the inner sphere, shown in green,
* `2` for the intermediate shell, shown in yellow, and
* `3` for the outer shell, shown in magenta.

A very coarse (`7 x 7 x 7`) segmentation, taken as a midline cut plane
may appear as

<style>
    .grid {
        display: grid;
        grid-template-columns: repeat(7, 50px);
        grid-template-rows: repeat(7, 50px);
        gap: 5px;
    }
    .cell {
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 24px;
        color: white;
    }
    .zero {
        background-color: gray;
    }
    .one {
        /* background-color: green; */
        background-color: rgb(0, 255, 0); /* RGB value for green */
        color: black;  /* text color */
    }
    .two {
        background-color: yellow;
        color: black;  /* text color */
    }
    .three {
        background-color: magenta;
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

This very coarse image, at two higher resolutions of (`24 x 24 x 24`) and
(`48 x 48 x 48`) is shown below:

![spheres_cont_cut](analysis/sphere_with_shells/img/spheres_cont_cut.png)
