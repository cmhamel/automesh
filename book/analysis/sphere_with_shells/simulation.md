# Simulation

Work in progress.

## Boundary Conditions

We impose an angular acceleration pulse to the outermost shell.
The angular acceleration has the form

$$\alpha(t) := \alpha_{\max} \exp \left( 1 - \frac{1}{1 - \left(\frac{2t}{\Delta t} - 1 \right)^2}\right) \hspace{0.5cm} {\rm for} \hspace{0.5cm} 0 \leq t \leq \Delta t $$

and $\alpha(t) = 0$ is zero otherwise.  This pulse, referred to as the **bump function**, is continuously differentiable, with peak angular acceleration at $\Delta t / 2$.

With $\alpha_{\max} := $ 8.0 krad/s^2, and $\Delta t := 8.0$ ms, we can create the angular acceleration boundary condition (with corresponding angular velocity) [^Carlsen_2021] plots:

| Angular Acceleration | Angular Velocity |
| :---: | :---: |
| ![](img/AngAccel.png) | ![](img/AngVel.png) |
> Figure: Angular acceleration and corresponding angular velocity time history.

* [Angular velocity tabulated data](https://1drv.ms/u/s!ApVSeeLlvsE8g_4yrDrMBjYM28vt6A?e=reeUyW):  The angular velocity curve has column data as (time, magnitude) in (sec, rad/s).
* [Angular acceleration tabulated data](https://1drv.ms/u/s!ApVSeeLlvsE8g_4xLyBDaZDDXvh7iw?e=iikM6v):  The standardized angular acceleration load curve has column data as (time, magnitude) in (sec, krad/s^2).  The function used to generated the curve, from Equation (1) of [^Carlsen_2021], is

The peak angular acceleration ocurrs at $t=\Delta t / 2$ (which occurs in the tabular data at data point 4144, values (0.00414310, 7.99999997)).

## Meshes

We consider three simulations using the following three meshes:

* `spheres_reolution_2.exo`
* `spheres_reolution_3.exo`
* `spheres_reolution_4.exo`

We do not use the `spheres_resolution_1.exo` because the outer shell layer is not closed.  We place these three meshes on the HPC in the shared folder called `~/autotwin/ssm/geometry/sphere/`.

```sh
# manual copy from local to HPC
# macOS local finder, command+K to launch "Connect to Server"
# smb://cee/chovey
# copy [local]~/autotwin/automesh/book/analysis/sphere_with_shells/*.exo
# to
# [HPC]~/autotwin/ssm/geometry/sphere/
```

## Solver

Request resources:

```sh
mywcid # see what wcid resources are available, gives an FYxxxxxx number
```

```sh
#request 1 interactive node for four hours with wcid account FY180100
salloc -N1 --time=4:00:00 --account=FY180100
```

See idle nodes:

```sh
sinfo
```

Check the input deck:

```sh
#!/bin/bash

echo "This is submit_check"
echo "module purge"
module purge

echo "module load sierra"
module load sierra

# new, run this first
export PSM2_DEVICES='shm,self'

IFILE="sr4.i"

echo "Check syntax of input deck: $IFILE"
adagio --check-syntax -i $IFILE  # to check syntax of input deck

# echo "Check syntax of input deck ($IFILE) and mesh loading"
adagio --check-input  -i $IFILE  # to check syntax of input deck and mesh load
```

Clean the result files:

```sh
#!/bin/bash

echo "This is submit_clean"
rm batch_*
rm sierra_batch_*
rm *.e.*
rm epu.log
rm *.g.*
rm *.err
rm g.rsout.*
```

Submit script:

```sh
#!/bin/bash

echo "This is submit_script"
module purge
module load sierra
module load seacas

# PROCS=16
PROCS=160
# PROCS=320
# PROCS=336

# geometry and mesh file
GFILE="../../geometry/sphere/spheres_resolution_4.exo"

decomp --processors $PROCS $GFILE

IFILE="sr4.i"

# queues
# https://wiki.sandia.gov/pages/viewpage.action?pageId=1359570410#SlurmDocumentation-Queues
# short can be used for nodes <= 40 and wall time <= 4:00:00 (4 hours)
# batch, the default queue, wall time <= 48 h
# long, wall time <= 96 h, eclipse 256 nodes

# https://wiki.sandia.gov/display/OK/Slurm+Documentation
# over 4 hours, then need to remove 'short' from the --queue-name
#
# sierra -T 00:20:00 --queue-name batch,short --account FY180042 -j $PROCS --job-name $IFILE --run adagio -i $IFILE
sierra -T 04:00:00 --queue-name batch,short --account FY180042 -j $PROCS --job-name $IFILE --run adagio -i $IFILE
# sierra -T 06:00:00 --queue-name batch --account FY180042 -j $PROCS --job-name $IFILE --run adagio -i $IFILE
# sierra -T 24:00:00 --queue-name batch --account FY180042 -j $PROCS --job-name $IFILE --run adagio -i $IFILE
```

Monitor the job:

```sh
# monitoring
squeue -u chovey
tail foo.log
tail -f foo.log # interactive monitor
```

Cancel the job:

```sh
scancel JOB_ID
scancel -u chovey # cancel all jobs
```

## References

[^Carlsen_2021]: Carlsen RW, Fawzi AL, Wan Y, Kesari H, Franck C. A quantitative relationship between rotational head kinematics and brain tissue strain from a 2-D parametric finite element analysis. Brain Multiphysics. 2021 Jan 1;2:100024.  [paper](https://1drv.ms/b/s!ApVSeeLlvsE8g9tyGKINkyp_5cb1hA?e=G9XGIZ)
